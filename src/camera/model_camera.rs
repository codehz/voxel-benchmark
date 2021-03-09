use glam::{
    f32::{Mat4, Vec3},
    Quat,
};

use super::{Camera, CameraCreation, CameraInput};

pub struct ModelCamera {
    x: f32,
    y: f32,
    z: f32,
    width: usize,
    length: usize,
    height: usize,
}

impl CameraCreation for ModelCamera {
    fn new(width: usize, height: usize, length: usize) -> Self {
        Self {
            x: 0f32,
            y: (height / 2) as f32,
            z: (length.max(width) as f32) * -1.0,
            width,
            height,
            length,
        }
    }
}

impl Camera for ModelCamera {
    fn get_matrix(&self) -> [[f32; 4]; 4] {
        let center = Vec3::new(
            (self.width / 2) as f32,
            (self.height / 2) as f32,
            (self.length / 2) as f32,
        );
        let rot = Quat::from_rotation_y(self.x);
        let eye = Mat4::from_rotation_translation(rot, center * Vec3::new(1.0, 0.0, 1.0))
            .transform_point3(Vec3::new(0.0, self.y, self.z));
        Mat4::look_at_rh(eye, center, Vec3::new(0.0, 1.0, 0.0)).to_cols_array_2d()
    }

    fn feed_input(&mut self, input: CameraInput) {
        match input {
            CameraInput::Move([x, y, z]) => {
                self.x += x * 0.01;
                self.x = self.x.rem_euclid((360.0f32).to_radians());
                self.y += y * 0.2;
                self.z += z * 0.1;
            }
        }
    }
}
