use byteorder::{BigEndian, ReadBytesExt};
use eframe::{
    egui, egui_glow,
    egui_glow::glow,
    epaint::{mutex::Mutex, PaintCallback},
};
use std::{collections::BTreeSet, io::Cursor, mem::transmute, sync::Arc};

const ROTATION_SENSITIVITY: f32 = 0.007;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ColorId(u32);
impl ColorId {
    fn none() -> Self {
        Self(0)
    }

    fn to_color(&self) -> Color {
        let bytes: [u8; 4] = unsafe { transmute(self.0.to_be()) };
        Color::new(bytes[0], bytes[1], bytes[2], 255)
    }

    fn from_color(color: &Color) -> Self {
        let mut reader = Cursor::new(vec![0, color.r, color.g, color.b]);
        Self(reader.read_u32::<BigEndian>().unwrap())
    }

    fn from_u32(id: u32) -> Self {
        Self(id << 8)
    }
}
impl Default for ColorId {
    fn default() -> Self {
        Self::none()
    }
}
impl FromCpuMaterial for ColorId {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
    }
}
impl Material for ColorId {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        r#"
            uniform vec4 surfaceColor;

            layout (location = 0) out vec4 outColor;

            void main()
            {
                outColor = surfaceColor;
            }
        "#
        .to_owned()
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform("surfaceColor", self.to_color());
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR_AND_DEPTH,
            depth_test: DepthTest::Less,
            blend: Blend::Disabled,
            cull: Cull::Back,
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}

struct ColorIdSource {
    seed: u32,
}
impl ColorIdSource {
    pub fn new() -> Self {
        Self { seed: 0 }
    }

    pub fn next(&mut self) -> ColorId {
        self.seed += 1;
        ColorId::from_u32(self.seed)
    }
}

pub struct PartEditor {
    id_source: ColorIdSource,
    rotation: Quaternion<f32>,
    scene: Arc<Mutex<ThreeDApp>>,
}
impl PartEditor {
    pub fn new(gl: GlowContext) -> Self {
        let mut id_source = ColorIdSource::new();
        Self {
            rotation: Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), Rad(0.0)),
            scene: Arc::new(Mutex::new(ThreeDApp::new(gl.clone(), &mut id_source))),
            id_source,
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
                    Vector3::new(-dy, dx, 0.0).normalize(),
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
                    scene.render(frame_input, rotation);
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
                let x = (mouse.0 - rect.min.x) * 1.5;
                let y = (rect.max.y - mouse.1) * 1.5;

                println!("RECT {:?}", rect);
                println!("MOUSE {} {}", x, y);

                let pick = self.scene.lock().read_color_id(x as u32, y as u32);

                println!("PICK {:#?}", pick);
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
        unsafe {
            use glow::HasContext as _;
            context.disable(glow::BLEND); // GL_BLEND
            context.disable(glow::DITHER); // GL_DITHER
                                           //context.disable(glow::FOG); // GL_FOG
                                           //context.disable(glow::LIGHTING); // GL_LIGHTING
            context.disable(glow::TEXTURE_1D); // GL_TEXTURE_1D
            context.disable(glow::TEXTURE_2D); // GL_TEXTURE_2D
            context.disable(glow::TEXTURE_3D); // GL_TEXTURE_3D

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
    model: Model<ColorId>,
    id_color_texture: Texture2D,
    id_depth_texture: DepthTexture2D,
}

impl ThreeDApp {
    fn new(gl: std::sync::Arc<glow::Context>, id_source: &mut ColorIdSource) -> Self {
        let context = Context::from_gl_context(gl).unwrap();

        // Create a camera
        let position = vec3(0.0, 0.0, -15.0); // Camera position
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

        //let x = loaded.deserialize("gizmo.obj").unwrap();

        let mut gizmo =
            Model::<ColorId>::new(&context, &loaded.deserialize("gizmo.obj").unwrap()).unwrap();

        for gm in gizmo.iter_mut() {
            gm.material = id_source.next();
            /*
            gm.material = ColorMaterial {
                color: id_source.next().to_color(),
                texture: None,
                is_transparent: false,
                render_states: RenderStates {
                    write_mask: WriteMask::COLOR_AND_DEPTH,
                    depth_test: gm.material.render_states.depth_test,
                    /*
                    blend: Blend::Enabled {
                        source_rgb_multiplier: BlendMultiplierType::One,
                        source_alpha_multiplier: BlendMultiplierType::One,
                        destination_rgb_multiplier: BlendMultiplierType::Zero,
                        destination_alpha_multiplier: BlendMultiplierType::Zero,
                        rgb_equation: BlendEquationType::Max,
                        alpha_equation: BlendEquationType::Max,
                    },
                    */
                    blend: Blend::Disabled,
                    cull: Cull::Back,
                },
            };
            */
        }

        let (id_color_texture, id_depth_texture) = Self::new_id_textures(&context, 1, 1);

        Self {
            id_color_texture,
            id_depth_texture,
            context,
            camera,
            model: gizmo,
        }
    }

    pub(crate) fn read_color_id(&mut self, x: u32, y: u32) -> Option<ColorId> {
        let color = self
            .id_color_texture
            .as_color_target(None)
            .read_partially(ScissorBox {
                x: x as i32,
                y: y as i32,
                width: 1,
                height: 1,
            });

        let colors: BTreeSet<Color> = BTreeSet::from_iter(color.iter().cloned());

        println!("PICK");
        for color in colors.iter() {
            println!("{:?}", color);
        }

        color.get(0).map(|color| ColorId::from_color(color))
    }

    pub fn render(
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

        self.render_id_textures(&frame_input);

        frame_input.screen.copy_partially_from(
            frame_input.viewport.into(),
            ColorTexture::Single(&self.id_color_texture),
            DepthTexture::Single(&self.id_depth_texture),
            frame_input.viewport,
            WriteMask::default(),
        );

        frame_input.screen.into_framebuffer() // Take back the screen fbo, we will continue to use it.
    }

    pub fn render_id_textures(&mut self, frame_input: &FrameInput<'_>) {
        if frame_input.viewport.width != self.id_color_texture.width()
            || frame_input.viewport.height != self.id_color_texture.height()
        {
            let (id_color_texture, id_depth_texture) = Self::new_id_textures(
                &self.context,
                frame_input.viewport.width,
                frame_input.viewport.height,
            );

            self.id_color_texture = id_color_texture;
            self.id_depth_texture = id_depth_texture;
        }

        /*
        unsafe {
            self.context.disable(3042); // GL_BLEND
            self.context.disable(3024); // GL_DITHER
            self.context.disable(2912); // GL_FOG
            self.context.disable(2896); // GL_LIGHTING
            self.context.disable(3552); // GL_TEXTURE_1D
            self.context.disable(3552); // GL_TEXTURE_2D
            self.context.disable(32879); // GL_TEXTURE_3D
            self.context.disable(36281); // GL_FRAMEBUFFER_SRGB

            self.context.disable(egui_glow::glow::FRAMEBUFFER_SRGB);
        }
        */

        // Render offscreen
        RenderTarget::new(
            self.id_color_texture.as_color_target(None),
            self.id_depth_texture.as_depth_target(),
        )
        .clear(ClearState::default())
        .render(&self.camera, &self.model, &[]);
    }

    fn new_id_textures(context: &Context, width: u32, height: u32) -> (Texture2D, DepthTexture2D) {
        (
            Texture2D::new_empty::<[f32; 3]>(
                context,
                width,
                height,
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            ),
            DepthTexture2D::new::<f32>(
                context,
                width,
                height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            ),
        )
    }
}
