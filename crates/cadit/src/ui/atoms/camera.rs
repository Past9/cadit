use three_d::{vec3, Angle, Camera, Deg, InnerSpace, Vec3, Viewport};

#[derive(Clone, Copy, PartialEq)]
pub enum CameraMode {
    Perspective,
    Orthographic,
}

pub struct CameraProps {
    position: Vec3,
    target: Vec3,
    fov_y: Deg<f32>,
    mode: CameraMode,
    viewport: Viewport,
    camera: Option<Camera>,
}
impl CameraProps {
    pub fn new(
        position: Vec3,
        target: Vec3,
        fov_y: Deg<f32>,
        mode: CameraMode,
        viewport: Viewport,
    ) -> Self {
        Self {
            position,
            target,
            fov_y,
            mode,
            viewport,
            camera: None,
        }
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        if viewport != self.viewport {
            self.camera = None;
            self.viewport = viewport;
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        if position != self.position {
            self.camera = None;
            self.position = position;
        }
    }

    pub fn set_target(&mut self, target: Vec3) {
        if target != self.target {
            self.camera = None;
            self.target = target;
        }
    }

    pub fn set_fov_y(&mut self, fov_y: Deg<f32>) {
        if fov_y != self.fov_y {
            self.camera = None;
            self.fov_y = fov_y;
        }
    }

    pub fn set_mode(&mut self, mode: CameraMode) {
        if mode != self.mode {
            self.camera = None;
            self.mode = mode;
        }
    }

    pub fn camera(&mut self) -> &mut Camera {
        if let Some(ref mut camera) = self.camera {
            camera
        } else {
            let camera = match self.mode {
                CameraMode::Perspective => self.create_perspective(),
                CameraMode::Orthographic => self.create_orthographic(),
            };

            self.camera.get_or_insert(camera)
        }
    }

    fn dist(&self) -> f32 {
        (self.position - self.target).magnitude()
    }

    fn height(&self) -> f32 {
        (self.fov_y / 2.0).tan() * self.dist() * 2.0
    }

    fn create_orthographic(&self) -> Camera {
        Camera::new_orthographic(
            self.viewport,
            self.position,
            self.target,
            vec3(0.0, 1.0, 0.0),
            self.height(),
            0.1,
            1000.0,
        )
    }

    fn create_perspective(&self) -> Camera {
        Camera::new_perspective(
            self.viewport,
            self.position,
            self.target,
            vec3(0.0, 1.0, 0.0),
            self.fov_y,
            0.1,
            1000.0,
        )
    }
}
