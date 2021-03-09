pub enum CameraInput {
    Move([f32; 3]),
}

pub trait Camera {
    fn get_matrix(&self) -> [[f32; 4]; 4];
    fn feed_input(&mut self, input: CameraInput);
}

pub trait CameraCreation: Camera {
    fn new(width: usize, height: usize, length: usize) -> Self;
}

pub mod model_camera;
