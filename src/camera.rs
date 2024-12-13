use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect_ratio: f32,
    pub fov: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_proj_matrix(&self) -> Matrix4<f32> {
        let view = Matrix4::look_at_rh(&self.eye, &self.target, &self.up);
        let proj = Perspective3::new(self.aspect_ratio, self.fov, self.znear, self.zfar).to_homogeneous();
        proj * view
    }
}