use crate::math::vector3d::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Matrix3x3 {
    pub m: [[f32; 3]; 3],
}

impl Matrix3x3 {
    pub fn new(
        m00: f32, m01: f32, m02: f32,
        m10: f32, m11: f32, m12: f32,
        m20: f32, m21: f32, m22: f32,
    ) -> Self {
        Self {
            m: [
                [m00, m01, m02],
                [m10, m11, m12],
                [m20, m21, m22],
            ]
        }
    }

    pub fn identity() -> Self {
        Self::new(
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        )
    }

    pub fn multiply_vec(&self, v: &Vec3) -> Vec3 {
        Vec3::new(
            self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2] * v.z,
            self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2] * v.z,
            self.m[2][0] * v.x + self.m[2][1] * v.y + self.m[2][2] * v.z,
        )
    }

    pub fn determinant(&self) -> f32 {
        self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1])
      - self.m[0][1] * (self.m[1][0] * self.m[2][2] - self.m[1][2] * self.m[2][0])
      + self.m[0][2] * (self.m[1][0] * self.m[2][1] - self.m[1][1] * self.m[2][0])
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        
        if det.abs() < 1e-6 {
            return None;
        }

        let inv_det = 1.0 / det;
        let m = &self.m;

        Some(Self::new(
            (m[1][1] * m[2][2] - m[1][2] * m[2][1]) * inv_det,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) * inv_det,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) * inv_det,
            
            (m[1][2] * m[2][0] - m[1][0] * m[2][2]) * inv_det,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) * inv_det,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) * inv_det,
            
            (m[1][0] * m[2][1] - m[1][1] * m[2][0]) * inv_det,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) * inv_det,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) * inv_det,
        ))
    }
}