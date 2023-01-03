use cgmath::{EuclideanSpace, InnerSpace, SquareMatrix};

use super::cgmath_types::*;
use cgmath::Angle;

#[derive(Debug, Clone)]
pub enum ProjectionType {
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
    viewport_in_pixels: [u32; 2],
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
        viewport_in_pixels: [u32; 2],
        position: Point3,
        direction: Vec3,
        up: Vec3,
        height: f32,
        near_dist: f32,
        far_dist: f32,
    ) -> Self {
        let mut camera = Camera::create(
            viewport_in_pixels.clone(),
            near_dist,
            far_dist,
            ProjectionType::Orthographic { height },
        );
        camera.orient(position, direction, up);
        camera.update_projection();
        camera
    }

    pub fn projection_matrix(&self) -> Mat4 {
        self.perspective_matrix() * self.view_matrix()
    }

    pub fn create_perspective(
        viewport_in_pixels: [u32; 2],
        position: Point3,
        direction: Vec3,
        up: Vec3,
        fov_y: Rad,
        near_dist: f32,
        far_dist: f32,
    ) -> Self {
        let mut camera = Camera::create(
            viewport_in_pixels,
            near_dist,
            far_dist,
            ProjectionType::Perspective { fov_y },
        );
        camera.orient(position, direction, up);
        camera.update_projection();
        camera
    }

    pub fn set_viewport_in_pixels(&mut self, viewport_in_pixels: [u32; 2]) {
        self.viewport_in_pixels = viewport_in_pixels;
        self.update_projection();
    }

    fn update_projection(&mut self) {
        match self.projection_type {
            ProjectionType::Orthographic { height } => {
                let width = height * self.aspect();
                self.perspective_matrix = Self::make_ortho_matrix(
                    -0.5 * width,
                    0.5 * width,
                    -0.5 * height,
                    0.5 * height,
                    self.near_dist,
                    self.far_dist,
                );
            }
            ProjectionType::Perspective { fov_y } => {
                self.perspective_matrix = Self::make_perspective_matrix(
                    fov_y,
                    self.aspect(),
                    self.near_dist,
                    self.far_dist,
                );
            }
        };
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

    fn make_ortho_matrix(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Mat4 {
        let c0r0 = 2.0 / (right - left);
        let c0r1 = 0.0;
        let c0r2 = 0.0;
        let c0r3 = 0.0;

        let c1r0 = 0.0;
        let c1r1 = 2.0 / (top - bottom);
        let c1r2 = 0.0;
        let c1r3 = 0.0;

        let c2r0 = 0.0;
        let c2r1 = 0.0;
        let c2r2 = 1.0 / (far - near);
        let c2r3 = 0.0;

        let c3r0 = (right + left) / (right - left);
        let c3r1 = (top + bottom) / (top - bottom);
        let c3r2 = -near / (far - near);
        let c3r3 = 1.0;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
    }

    fn make_perspective_matrix(fov_y: Rad, aspect: f32, near: f32, far: f32) -> Mat4 {
        let f = Rad::cot(fov_y / 2.0);

        let c0r0 = f / aspect;
        let c0r1 = 0.0;
        let c0r2 = 0.0;
        let c0r3 = 0.0;

        let c1r0 = 0.0;
        let c1r1 = f;
        let c1r2 = 0.0;
        let c1r3 = 0.0;

        let c2r0 = 0.0;
        let c2r1 = 0.0;
        let c2r2 = 1.0 / (far - near);
        let c2r3 = 1.0;

        let c3r0 = 0.0;
        let c3r1 = 0.0;
        let c3r2 = -near / (far - near);
        let c3r3 = 0.0;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat4::new(
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        )
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
            -pos.dot(s), -pos.dot(u), -pos.dot(f), 1.0,
        )
    }

    pub fn near_dist(&self) -> f32 {
        self.near_dist
    }

    pub fn far_dist(&self) -> f32 {
        self.far_dist
    }

    pub fn aspect(&self) -> f32 {
        self.viewport_in_pixels[0] as f32 / self.viewport_in_pixels[1] as f32
    }

    pub fn view_matrix(&self) -> &Mat4 {
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

    fn create(
        viewport_in_pixels: [u32; 2],
        near_dist: f32,
        far_dist: f32,
        projection_type: ProjectionType,
    ) -> Self {
        Camera {
            viewport_in_pixels,
            projection_type,
            near_dist,
            far_dist,
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
