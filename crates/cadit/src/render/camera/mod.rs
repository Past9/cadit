use cgmath::{EuclideanSpace, InnerSpace, SquareMatrix};
use eframe::epaint::PaintCallbackInfo;
use vulkano::pipeline::graphics::viewport::Viewport;

use super::cgmath_types::*;

#[derive(PartialEq, Clone, Debug)]
pub struct CameraViewport {
    pub origin: [u32; 2],
    pub dimensions: [u32; 2],
}
impl CameraViewport {
    pub fn zero() -> Self {
        Self {
            origin: [0, 0],
            dimensions: [0, 0],
        }
    }

    pub fn aspect(&self) -> f32 {
        self.dimensions[0] as f32 / self.dimensions[1] as f32
    }

    pub fn from_info(info: &PaintCallbackInfo) -> Self {
        let vp = info.viewport_in_pixels();
        Self {
            origin: [vp.left_px as u32, vp.top_px as u32],
            dimensions: [vp.width_px as u32, vp.height_px as u32],
        }
    }

    pub fn to_vulkan_viewport(&self) -> Viewport {
        Viewport {
            origin: [self.origin[0] as f32, self.origin[1] as f32],
            dimensions: [self.dimensions[0] as f32, self.dimensions[1] as f32],
            depth_range: 0.0..1.0,
        }
    }
}

enum ProjectionType {
    Orthographic { height: f32 },
    Perspective { fov_y: Rad },
}

struct Frustum {
    near: Vec4,
    far: Vec4,
    right: Vec4,
    left: Vec4,
    top: Vec4,
    bottom: Vec4,
}
impl Frustum {
    pub fn zero() -> Self {
        Frustum {
            near: vec4(0.0, 0.0, 0.0, 0.0),
            far: vec4(0.0, 0.0, 0.0, 0.0),
            right: vec4(0.0, 0.0, 0.0, 0.0),
            left: vec4(0.0, 0.0, 0.0, 0.0),
            top: vec4(0.0, 0.0, 0.0, 0.0),
            bottom: vec4(0.0, 0.0, 0.0, 0.0),
        }
    }
}

pub struct Camera {
    viewport: CameraViewport,
    projection_type: ProjectionType,
    near_dist: f32,
    far_dist: f32,
    position: Point3,
    direction: Vec3,
    up: Vec3,
    view_matrix: Mat4,
    perspective_matrix: Mat4,
    screen_to_ray_matrix: Mat4,
    frustum: Frustum,
}
impl Camera {
    pub fn create_orthographic(
        viewport: CameraViewport,
        position: Point3,
        direction: Vec3,
        up: Vec3,
        near_dist: f32,
        far_dist: f32,
    ) -> Self {
        let mut camera = Camera::create(viewport.clone());
        camera.orient(position, direction, up);
        camera.set_orthograpic(viewport, near_dist, far_dist);
        camera
    }

    pub fn create_perspective(
        viewport: CameraViewport,
        position: Point3,
        direction: Vec3,
        up: Vec3,
        fov_y: Rad,
        near_dist: f32,
        far_dist: f32,
    ) -> Self {
        let mut camera = Camera::create(viewport);
        camera.orient(position, direction, up);
        camera.set_perspective(fov_y, near_dist, far_dist);
        camera
    }

    pub fn set_orthograpic(&mut self, viewport: CameraViewport, near_dist: f32, far_dist: f32) {
        self.viewport = viewport;
        self.near_dist = near_dist;
        self.far_dist = far_dist;
        let width = self.viewport.dimensions[0] as f32;
        let height = self.viewport.dimensions[1] as f32;
        self.projection_type = ProjectionType::Orthographic { height };
        println!("WH {} {}", width, height);
        self.perspective_matrix = cgmath::ortho(
            -0.5 * width,
            0.5 * width,
            -0.5 * height,
            0.5 * height,
            near_dist,
            far_dist,
        );
        println!("PM {:#?}", self.perspective_matrix);
        self.update_screen_to_ray_matrix();
        self.update_frustum();
    }

    pub fn set_perspective(&mut self, fov_y: Rad, near_dist: f32, far_dist: f32) {
        self.near_dist = near_dist;
        self.far_dist = far_dist;
        self.projection_type = ProjectionType::Perspective { fov_y };
        self.perspective_matrix =
            cgmath::perspective(fov_y, self.viewport.aspect(), near_dist, far_dist);
        self.update_screen_to_ray_matrix();
        self.update_frustum();
    }

    pub fn orient(&mut self, position: Point3, direction: Vec3, up: Vec3) {
        self.position = position;
        self.direction = direction;
        self.up = up;
        self.view_matrix = Self::make_view_matrix(self.position, self.direction, self.up);
        self.update_screen_to_ray_matrix();
        self.update_frustum();
    }

    fn make_view_matrix(pos: Point3, dir: Vec3, up: Vec3) -> Mat4 {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = f.cross(s);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat4::new(
            s.x.clone(), u.x.clone(), f.x.clone(), 0.0,
            s.y.clone(), u.y.clone(), f.y.clone(), 0.0,
            s.z.clone(), u.z.clone(), f.z.clone(), 0.0,
            pos.dot(s), pos.dot(u), pos.dot(f), 1.0,
        )
    }

    pub fn near_dist(&self) -> f32 {
        self.near_dist
    }

    pub fn far_dist(&self) -> f32 {
        self.far_dist
    }

    pub fn viewport(&self) -> &CameraViewport {
        &self.viewport
    }

    pub fn view_matrix(&self) -> &Mat4 {
        //println!("{:?} {:?} {:?}", self.position, self.direction, self.up);
        &self.view_matrix
    }

    pub fn perspective_matrix(&self) -> &Mat4 {
        &self.perspective_matrix
    }

    fn update_screen_to_ray_matrix(&mut self) {
        let mut view_matrix = self.view_matrix;
        view_matrix[3] = vec4(0.0, 0.0, 0.0, 1.0);
        self.screen_to_ray_matrix = (self.perspective_matrix * view_matrix).invert().unwrap();
    }

    fn update_frustum(&mut self) {
        let m = self.perspective_matrix * self.view_matrix;
        self.frustum = Frustum {
            near: vec4(m.x.w + m.x.x, m.y.w + m.y.x, m.z.w + m.z.x, m.w.w + m.w.x),
            far: vec4(m.x.w - m.x.x, m.y.w - m.y.x, m.z.w - m.z.x, m.w.w - m.w.x),
            right: vec4(m.x.w + m.x.y, m.y.w + m.y.y, m.z.w + m.z.y, m.w.w + m.w.y),
            left: vec4(m.x.w - m.x.y, m.y.w - m.y.y, m.z.w - m.z.y, m.w.w - m.w.y),
            top: vec4(m.x.w + m.x.z, m.y.w + m.y.z, m.z.w + m.z.z, m.w.w + m.w.z),
            bottom: vec4(m.x.w - m.x.z, m.y.w - m.y.z, m.z.w - m.z.z, m.w.w - m.w.z),
        };
    }

    fn create(viewport: CameraViewport) -> Self {
        Camera {
            viewport,
            projection_type: ProjectionType::Orthographic { height: 0.0 },
            near_dist: 0.0,
            far_dist: 0.0,
            position: point3(0.0, 0.0, 0.0),
            direction: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 0.0),
            view_matrix: Mat4::identity(),
            perspective_matrix: Mat4::identity(),
            screen_to_ray_matrix: Mat4::identity(),
            frustum: Frustum::zero(),
        }
    }
}
