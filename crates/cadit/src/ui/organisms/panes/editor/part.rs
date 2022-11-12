use eframe::{egui, egui_glow, egui_glow::glow, epaint::PaintCallback};
use std::{ops::Deref, sync::Arc};

const ROTATION_SENSITIVITY: f32 = 0.007;

pub struct PartEditorState {
    rotation: Quaternion<f32>,
}
impl PartEditorState {
    pub fn new() -> Self {
        Self {
            rotation: Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), Rad(0.0)),
        }
    }
}

pub struct PartEditor<'a> {
    pub(crate) state: &'a mut PartEditorState,
}
impl<'a> PartEditor<'a> {
    pub fn with_state(state: &'a mut PartEditorState) -> Self {
        Self { state }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());

            let dx = response.drag_delta().x;
            let dy = response.drag_delta().y;

            if dy != 0.0 || dx != 0.0 {
                self.state.rotation = Quaternion::from_axis_angle(
                    Vector3::new(dy, dx, 0.0).normalize(),
                    Rad(Vector3::new(dx, dy, 0.0).magnitude() * ROTATION_SENSITIVITY),
                ) * self.state.rotation;
            }

            println!("{:#?}", ui.input().events);

            let rotation = self.state.rotation;

            let callback = PaintCallback {
                rect,
                callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                    with_three_d(painter.gl(), |three_d| {
                        three_d.frame(FrameInput::new(&three_d.context, &info, painter), rotation)
                    });
                })),
            };

            ui.painter().add(callback);
        });
    }
}

/// We get a [`glow::Context`] from `eframe` and we want to construct a [`ThreeDApp`].
///
/// Sadly we can't just create a [`ThreeDApp`] in [`MyApp::new`] and pass it
/// to the [`egui::PaintCallback`] because [`glow::Context`] isn't `Send+Sync` on web, which
/// [`egui::PaintCallback`] needs. If you do not target web, then you can construct the [`ThreeDApp`] in [`MyApp::new`].
fn with_three_d<R>(
    gl: &std::sync::Arc<egui_glow::glow::Context>,
    f: impl FnOnce(&mut ThreeDApp) -> R,
) -> R {
    use std::cell::RefCell;
    thread_local! {
        pub static THREE_D: RefCell<Option<ThreeDApp>> = RefCell::new(None);
    }

    THREE_D.with(|three_d| {
        let mut three_d = three_d.borrow_mut();
        let three_d = three_d.get_or_insert_with(|| ThreeDApp::new(gl.clone()));
        f(three_d)
    })
}

///
/// Translates from egui input to three-d input
///
pub struct FrameInput<'a> {
    screen: three_d::RenderTarget<'a>,
    viewport: three_d::Viewport,
    scissor_box: three_d::ScissorBox,
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
        let screen = painter.intermediate_fbo().map_or_else(
            || {
                RenderTarget::screen(
                    context,
                    info.viewport.width() as u32,
                    info.viewport.height() as u32,
                )
            },
            |fbo| {
                RenderTarget::from_framebuffer(
                    context,
                    info.viewport.width() as u32,
                    info.viewport.height() as u32,
                    fbo,
                )
            },
        );

        // Set where to paint
        let viewport = info.viewport_in_pixels();
        let viewport = Viewport {
            x: viewport.left_px.round() as _,
            y: viewport.from_bottom_px.round() as _,
            width: viewport.width_px.round() as _,
            height: viewport.height_px.round() as _,
        };

        // Respect the egui clip region (e.g. if we are inside an `egui::ScrollArea`).
        let clip_rect = info.clip_rect_in_pixels();
        let scissor_box = ScissorBox {
            x: clip_rect.left_px.round() as _,
            y: clip_rect.from_bottom_px.round() as _,
            width: clip_rect.width_px.round() as _,
            height: clip_rect.height_px.round() as _,
        };
        Self {
            screen,
            scissor_box,
            viewport,
        }
    }
}

///
/// Based on the `three-d` [Triangle example](https://github.com/asny/three-d/blob/master/examples/triangle/src/main.rs).
/// This is where you'll need to customize
///
use three_d::*;
pub struct ThreeDApp {
    context: Context,
    camera: Camera,
    //model: Gm<Mesh, ColorMaterial>,
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

        println!("LOADED {:#?}", loaded.keys());

        let mut gizmo =
            Model::<PhysicalMaterial>::new(&context, &loaded.deserialize("gizmo.obj").unwrap())
                .unwrap();

        println!("GOT GIZMO");

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

    pub fn frame(
        &mut self,
        frame_input: FrameInput<'_>,
        rotation: Quaternion<f32>,
    ) -> Option<glow::Framebuffer> {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        self.camera.set_viewport(frame_input.viewport);

        for model in self.model.iter_mut() {
            model.set_transformation(Mat4::from(rotation));
        }

        let lights = self
            .ambient_lights
            .iter()
            .map(|l| l as &dyn Light)
            .collect::<Vec<&dyn Light>>();

        // Get the screen render target to be able to render something on the screen
        frame_input
            .screen
            // Clear the color and depth of the screen render target
            .clear_partially(frame_input.scissor_box, ClearState::depth(1.0))
            // Render the triangle with the color material which uses the per vertex colors defined at construction
            .render_partially(frame_input.scissor_box, &self.camera, &self.model, &lights);

        frame_input.screen.into_framebuffer() // Take back the screen fbo, we will continue to use it.
    }
}
