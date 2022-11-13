use eframe::{
    egui, egui_glow,
    egui_glow::glow,
    epaint::{mutex::Mutex, PaintCallback},
};
use std::sync::Arc;

const ROTATION_SENSITIVITY: f32 = 0.007;

pub struct PartEditor {
    rotation: Quaternion<f32>,
    scene: Arc<Mutex<ThreeDApp>>,
}
impl PartEditor {
    pub fn new(gl: GlowContext) -> Self {
        Self {
            rotation: Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), Rad(0.0)),
            scene: Arc::new(Mutex::new(ThreeDApp::new(gl.clone()))),
        }
    }
}
impl Editor for PartEditor {
    fn title(&self) -> String {
        "Part editor".to_owned()
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());

            let dx = response.drag_delta().x;
            let dy = response.drag_delta().y;

            if dy != 0.0 || dx != 0.0 {
                self.rotation = Quaternion::from_axis_angle(
                    Vector3::new(dy, dx, 0.0).normalize(),
                    Rad(Vector3::new(dx, dy, 0.0).magnitude() * ROTATION_SENSITIVITY),
                ) * self.rotation;
            }

            let rotation = self.rotation;

            let scene = self.scene.clone();

            let paint_callback = PaintCallback {
                rect,
                callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                    let mut scene = scene.lock();
                    let context = &scene.context;
                    let frame_input = FrameInput::new(context, &info, painter);
                    scene.frame(frame_input, rotation);
                })),
            };

            //println!("{:#?}", ui.input().events);

            let mut mouse: Option<(f32, f32)> = None;

            for event in ui.input().events.iter() {
                match event {
                    egui::Event::PointerMoved(pos) => mouse = Some((pos.x, pos.y)),
                    _ => {}
                }
            }

            if let Some(mouse) = mouse {
                let scene = self.scene.lock();
                let picks = scene.pick(mouse);
                println!("PICKS {:#?}", picks);
            }

            ui.painter().add(paint_callback);
        });
    }
}

///
/// Translates from egui input to three-d input
///
pub struct FrameInput<'a> {
    screen: three_d::RenderTarget<'a>,
    viewport: three_d::Viewport,
}

impl FrameInput<'_> {
    pub fn new(
        context: &three_d::Context,
        info: &egui::PaintCallbackInfo,
        painter: &egui_glow::Painter,
    ) -> Self {
        use three_d::*;

        // Disable sRGB textures for three-d
        #[cfg(not(target_arch = "wasm32"))]
        #[allow(unsafe_code)]
        unsafe {
            use glow::HasContext as _;
            context.disable(glow::FRAMEBUFFER_SRGB);
        }

        // Constructs a screen render target to render the final image to
        let viewport = info.viewport_in_pixels();
        let screen = painter.intermediate_fbo().map_or_else(
            || {
                RenderTarget::screen(
                    context,
                    viewport.width_px.round() as u32,
                    viewport.height_px.round() as u32,
                )
            },
            |fbo| {
                RenderTarget::from_framebuffer(
                    context,
                    viewport.width_px.round() as u32,
                    viewport.height_px.round() as u32,
                    fbo,
                )
            },
        );

        // Set where to paint
        let viewport = Viewport {
            x: viewport.left_px.round() as _,
            y: viewport.from_bottom_px.round() as _,
            width: viewport.width_px.round() as _,
            height: viewport.height_px.round() as _,
        };

        Self { screen, viewport }
    }
}

///
/// Based on the `three-d` [Triangle example](https://github.com/asny/three-d/blob/master/examples/triangle/src/main.rs).
/// This is where you'll need to customize
///
use three_d::*;

use crate::ui::GlowContext;

use super::Editor;
pub struct ThreeDApp {
    context: Context,
    camera: Camera,
    model: Model<PhysicalMaterial>,
    ambient_lights: Vec<AmbientLight>,
}

impl ThreeDApp {
    pub fn new(gl: std::sync::Arc<glow::Context>) -> Self {
        let context = Context::from_gl_context(gl).unwrap();
        // Create a camera

        let position = vec3(0.0, 0.0, 15.0); // Camera position
        let target = vec3(0.0, 0.0, 0.0); // Look-at point
        let dist = (position - target).magnitude(); // Distance from camera origin to look-at point
        let fov_y = degrees(45.0); // Y-FOV for perspective camera
        let height = (fov_y / 2.0).tan() * dist * 2.0; // FOV-equivalent height for ortho camera

        // Ortho camera
        let camera = Camera::new_orthographic(
            Viewport::new_at_origo(1, 1),
            position,
            target,
            vec3(0.0, 1.0, 0.0),
            height,
            0.1,
            1000.0,
        );

        /*
        // Perspective camera
        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1),
            position,
            target,
            vec3(0.0, 1.0, 0.0),
            fov_y,
            0.1,
            1000.0,
        );
        */

        let mut loaded = three_d_asset::io::load(&["resources/assets/gizmo.obj"]).unwrap();

        let mut gizmo =
            Model::<PhysicalMaterial>::new(&context, &loaded.deserialize("gizmo.obj").unwrap())
                .unwrap();

        gizmo
            .iter_mut()
            .for_each(|g| g.material.render_states.cull = Cull::Back);

        let ambient = AmbientLight::new(&context, 1.0, Color::WHITE);

        Self {
            context,
            camera,
            model: gizmo,
            ambient_lights: vec![ambient],
        }
    }

    pub fn pick(&self, pixel: (f32, f32)) -> Vec<String> {
        self.model
            .iter()
            .filter_map(|m| {
                pick(&self.context, &self.camera, pixel, m).map(|_| m.material.name.to_owned())
            })
            .collect()
    }

    pub fn frame(
        &mut self,
        frame_input: FrameInput<'_>,
        rotation: Quaternion<f32>,
    ) -> Option<glow::Framebuffer> {
        // Ensure the camera viewport size matches the current window viewport
        // size which changes if the window is resized
        self.camera.set_viewport(Viewport {
            x: 0,
            y: 0,
            width: frame_input.viewport.width,
            height: frame_input.viewport.height,
        });

        for model in self.model.iter_mut() {
            model.set_transformation(Mat4::from(rotation));
        }

        let lights = self
            .ambient_lights
            .iter()
            .map(|l| l as &dyn Light)
            .collect::<Vec<&dyn Light>>();

        // Create offscreen textures to render the initial image to
        let mut color_texture = Texture2D::new_empty::<[u8; 4]>(
            &self.context,
            frame_input.viewport.width,
            frame_input.viewport.height,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        let mut depth_texture = DepthTexture2D::new::<f32>(
            &self.context,
            frame_input.viewport.width,
            frame_input.viewport.height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        // Render offscreen
        RenderTarget::new(
            color_texture.as_color_target(None),
            depth_texture.as_depth_target(),
        )
        .clear(ClearState::default())
        .render(&self.camera, &self.model, &lights);

        // Copy to the screen, applying FXAA
        frame_input
            .screen
            .copy_partially_from(
                frame_input.viewport.into(),
                ColorTexture::Single(&color_texture),
                DepthTexture::Single(&depth_texture),
                frame_input.viewport,
                WriteMask::default(),
            )
            .write_partially(
                ScissorBox {
                    x: frame_input.viewport.x,
                    y: frame_input.viewport.y,
                    width: frame_input.viewport.width,
                    height: frame_input.viewport.height,
                    //frame_input.viewport.into()
                },
                || {
                    (FxaaEffect {}).apply(&self.context, ColorTexture::Single(&color_texture));
                },
            );

        frame_input.screen.into_framebuffer() // Take back the screen fbo, we will continue to use it.
    }
}
