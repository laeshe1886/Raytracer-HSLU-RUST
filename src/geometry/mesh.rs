use crate::math::vector3d::Vec3;
use crate::math::ray::Ray;
use crate::geometry::hittable::{Hit, Hittable};
use crate::geometry::triangle::Triangle;
use crate::geometry::aabb::AABB;
use crate::geometry::bvh::BVHNode;
use crate::material::Material;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

pub struct Mesh {
    pub bvh_root: Arc<dyn Hittable>,
}

impl Mesh {
    /// Konstruktor, der eine OBJ-Datei und die zugehörige MTL-Datei einliest
    pub fn from_obj(file_path: &str, fallback_material: Material) -> Self {
        let mut triangles: Vec<Arc<dyn Hittable>> = Vec::new();
        let mut vertices = Vec::new();
        let mut normals = Vec::new(); // Speicher für Vertex Normals

        let path = Path::new(file_path);
        let file = File::open(path).expect("OBJ Datei nicht gefunden");
        let reader = BufReader::new(file);

        let mut materials: HashMap<String, Material> = HashMap::new();
        let mut current_material = fallback_material.clone();

        println!("Lade Mesh: {}...", file_path);

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() || parts[0].starts_with('#') { continue; }

            match parts[0] {
                "mtllib" => {
                    // Lade die MTL-Datei aus demselben Ordner wie die OBJ-Datei
                    let mtl_file_name = parts[1];
                    let mtl_path = path.parent().unwrap_or(Path::new("")).join(mtl_file_name);
                    materials = Self::load_materials(&mtl_path);
                },
                "usemtl" => {
                    // Wechsle das aktive Material für die folgenden Dreiecke
                    if let Some(mat) = materials.get(parts[1]) {
                        current_material = mat.clone();
                    }
                },
                "v" => {
                    vertices.push(Vec3::new(
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                        parts[3].parse().unwrap(),
                    ));
                },
                "vn" => {
                    // Normalenvektoren einlesen
                    normals.push(Vec3::new(
                        parts[1].parse().unwrap(),
                        parts[2].parse().unwrap(),
                        parts[3].parse().unwrap(),
                    ).normalize());
                },
                "f" => {
                    let mut v_indices = Vec::new();
                    let mut n_indices = Vec::new();

                    for part in &parts[1..] {
                        let sub_parts: Vec<&str> = part.split('/').collect();
                        // Vertex Index (OBJ zählt ab 1, Rust ab 0)
                        v_indices.push(sub_parts[0].parse::<usize>().unwrap() - 1);
                        
                        // Normalen Index extrahieren (falls vorhanden, Format: v/vt/vn oder v//vn)
                        if sub_parts.len() >= 3 && !sub_parts[2].is_empty() {
                            n_indices.push(sub_parts[2].parse::<usize>().unwrap() - 1);
                        }
                    }

                    // Face in Dreiecke zerlegen (Triangulierung)
                    for i in 1..v_indices.len() - 1 {
                        let (na, nb, nc) = if n_indices.len() == v_indices.len() {
                            (
                                Some(normals[n_indices[0]]), 
                                Some(normals[n_indices[i]]), 
                                Some(normals[n_indices[i+1]])
                            )
                        } else {
                            (None, None, None)
                        };

                        let tri = Triangle {
                            a: vertices[v_indices[0]],
                            b: vertices[v_indices[i]],
                            c: vertices[v_indices[i + 1]],
                            na, nb, nc,
                            material: current_material.clone(),
                        };
                        triangles.push(Arc::new(tri));
                    }
                }
                _ => {}
            }
        }

        println!("Mesh geladen ({} Dreiecke). Baue BVH...", triangles.len());
        let bvh_root = BVHNode::new(triangles);
        println!("BVH bereit.");

        Mesh { bvh_root }
    }

    /// Hilfsfunktion zum Parsen der .mtl Datei
    fn load_materials(path: &Path) -> HashMap<String, Material> {
        let mut materials = HashMap::new();
        
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                println!("Warnung: MTL-Datei nicht gefunden unter {:?}", path);
                return materials;
            }
        };

        let reader = BufReader::new(file);
        let mut current_name = String::new();
        
        let mut kd = Vec3::new(0.8, 0.8, 0.8);
        let mut ns = 10.0;
        let mut ni = 1.0;
        let mut d = 1.0;

        // Innere Funktion, um das zusammengebaute Material in die Map zu speichern
        let save_current_material = |name: &str, map: &mut HashMap<String, Material>, ni: f32, d: f32, kd: Vec3, ns: f32| {
            if name.is_empty() { return; }
            
            // Heuristik: Wenn der Brechungsindex > 1.0 ist oder das Material transparent ist (d < 1.0), machen wir Glas daraus
            let mat = if ni > 1.0 || d < 1.0 {
                Material::Dielectric {
                    refractive_index: if ni > 1.0 { ni } else { 1.5 },
                    // Wir leiten die Absorption grob aus der Objektfarbe (Kd) ab
                    absorption: Vec3::new(1.0 - kd.x, 1.0 - kd.y, 1.0 - kd.z) * 0.1,
                }
            } else {
                Material::Phong {
                    ambient: 0.1,
                    albedo: kd,
                    shininess: ns,
                    kd: 0.8,
                    ka: 1.0,
                    ks: 0.5,
                }
            };
            map.insert(name.to_string(), mat);
        };

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() || parts[0].starts_with('#') { continue; }

            match parts[0] {
                "newmtl" => {
                    // Speichere das vorherige Material, bevor ein neues beginnt
                    save_current_material(&current_name, &mut materials, ni, d, kd, ns);
                    current_name = parts[1].to_string();
                    
                    // Reset der Werte für das neue Material
                    kd = Vec3::new(0.8, 0.8, 0.8);
                    ns = 10.0;
                    ni = 1.0;
                    d = 1.0;
                },
                "Kd" => {
                    kd = Vec3::new(
                        parts[1].parse().unwrap_or(0.8),
                        parts[2].parse().unwrap_or(0.8),
                        parts[3].parse().unwrap_or(0.8),
                    );
                },
                "Ns" => { ns = parts[1].parse().unwrap_or(10.0); },
                "Ni" => { ni = parts[1].parse().unwrap_or(1.0); },
                "d" | "Tr" => { d = parts[1].parse().unwrap_or(1.0); },
                _ => {}
            }
        }
        // Das letzte Material in der Datei speichern
        save_current_material(&current_name, &mut materials, ni, d, kd, ns);

        println!("MTL geladen: {} Materialien gefunden.", materials.len());
        materials
    }
}

impl Hittable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.bvh_root.intersect(ray)
    }

    fn bounding_box(&self) -> AABB {
        self.bvh_root.bounding_box()
    }
}