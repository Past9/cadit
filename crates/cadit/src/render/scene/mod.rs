use cgmath::Zero;

use super::{
    camera::Camera,
    cgmath_types::{point3, vec3, Mat4, Point3, Quat, Vec3},
    lights::{AmbientLight, DirectionalLight, PointLight},
    model::Model,
};

pub struct SceneLights {
    ambient: Vec<AmbientLight>,
    directional: Vec<DirectionalLight>,
    point: Vec<PointLight>,
}

pub struct Orientation {
    offset: Vec3,
    rotation: Quat,
}
impl Orientation {
    pub fn zero() -> Self {
        Self {
            offset: vec3(0.0, 0.0, 0.0),
            rotation: Quat::zero(),
        }
    }

    pub fn set_offset(&mut self, offset: Vec3) {
        self.offset = offset;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    pub fn matrix(&self) -> Mat4 {
        Mat4::from_translation(self.offset) * Mat4::from(self.rotation)
    }
}

pub struct Scene {
    orientation: Orientation,
    lights: SceneLights,
    camera: Camera,
    models: Vec<Model>,
}
impl Scene {
    pub fn new(lights: SceneLights, camera: Camera, models: Vec<Model>) -> Self {
        Self {
            orientation: Orientation::zero(),
            lights,
            camera,
            models,
        }
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn orientation(&self) -> &Orientation {
        &self.orientation
    }

    pub fn orientation_mut(&mut self) -> &mut Orientation {
        &mut self.orientation
    }
}
