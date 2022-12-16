use super::scene::{ColorId, ColorIdSource, Scene, SceneObjectProps};
use crate::ui::GlowContext;
use eframe::{
    egui::{self, PointerButton},
    egui_glow,
    epaint::{mutex::Mutex, PaintCallback, Pos2, Rect},
};
use std::sync::Arc;
use three_d::{InnerSpace, Quaternion, Rad, Rotation3, Vector3};

const ROTATION_SENSITIVITY: f32 = 0.007;

#[derive(Clone, Debug)]
pub struct PointerButtonDown {
    pos: Pos2,
    button: PointerButton,
    down_at: std::time::Instant,
    modifiers: eframe::egui::Modifiers,
    scene_object: Option<ColorId>,
}

pub struct ObjectScene {
    id_source: ColorIdSource,
    rotation: Quaternion<f32>,
    scene: Arc<Mutex<Scene>>,
    scene_rect: egui::Rect,
    pointer_buttons_down: Vec<PointerButtonDown>,
    clicked: Option<ColorId>,
    rotated: bool,
}
impl ObjectScene {
    pub fn new(gl: GlowContext, rotation: Quaternion<f32>) -> Self {
        let mut id_source = ColorIdSource::new();

        Self {
            rotation,
            scene: Arc::new(Mutex::new(Scene::new(gl.clone(), &mut id_source))),
            id_source,
            scene_rect: egui::Rect {
                min: (0.0, 0.0).into(),
                max: (0.0, 0.0).into(),
            },
            pointer_buttons_down: Vec::new(),
            clicked: None,
            rotated: false,
        }
    }

    fn ui_pos_to_fbo_pos(&self, ui: &egui::Ui, ui_pos: Pos2) -> Pos2 {
        let pix_per_pt = ui.input().pixels_per_point;
        let x = (ui_pos.x - self.scene_rect.min.x) * pix_per_pt;
        let y = (self.scene_rect.max.y - ui_pos.y) * pix_per_pt;
        Pos2 { x, y }
    }

    pub fn get_rotation(&mut self) -> Quaternion<f32> {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }

    pub fn rect(&self) -> Rect {
        self.scene_rect
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.clicked = None;
        self.rotated = false;

        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());

            self.scene_rect = rect;

            let dx = response.drag_delta().x;
            let dy = response.drag_delta().y;

            if dy != 0.0 || dx != 0.0 {
                self.rotation = Quaternion::from_axis_angle(
                    Vector3::new(-dy, dx, 0.0).normalize(),
                    Rad(Vector3::new(dx, dy, 0.0).magnitude() * ROTATION_SENSITIVITY),
                ) * self.rotation;
                self.rotated = true;
            }

            let scene = self.scene.clone();

            let rotation = self.rotation;

            let paint_callback = PaintCallback {
                rect,
                callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                    let mut scene = scene.lock();
                    let context = &scene.context();
                    let frame_input = crate::render::FrameInput::new(context, &info, painter);
                    println!("{:?}", rotation);
                    scene.render(frame_input, rotation);
                })),
            };

            let mut scene = self.scene.lock();
            for event in ui.input().events.iter() {
                match event {
                    egui::Event::PointerMoved(pos) => {
                        let obj_id = scene.read_color_id(self.ui_pos_to_fbo_pos(ui, *pos));
                        scene.hover_object(obj_id);
                    }
                    egui::Event::PointerButton {
                        pos,
                        button,
                        pressed,
                        modifiers,
                    } => {
                        let obj_id = scene.read_color_id(self.ui_pos_to_fbo_pos(ui, *pos));

                        if *pressed {
                            self.pointer_buttons_down
                                .retain(|down| down.button != *button);

                            self.pointer_buttons_down.push(PointerButtonDown {
                                pos: *pos,
                                button: *button,
                                down_at: std::time::Instant::now(),
                                modifiers: modifiers.to_owned(),
                                scene_object: obj_id,
                            });
                        } else {
                            if let Some(obj_id) = obj_id {
                                let down = self
                                    .pointer_buttons_down
                                    .iter()
                                    .find(|down| down.scene_object == Some(obj_id));

                                if let Some(down) = down {
                                    let shift_select = down.modifiers.shift && modifiers.shift;

                                    scene.toggle_select_object(Some(obj_id), !shift_select);

                                    if scene.get_object(Some(obj_id)).is_some() {
                                        self.clicked = Some(obj_id);
                                    }
                                }
                            } else {
                                if !modifiers.shift {
                                    scene.deselect_all_objects();
                                }
                            }

                            self.pointer_buttons_down = Vec::new();
                        }
                    }
                    _ => {}
                }
            }

            ui.painter().add(paint_callback);
        });
    }

    pub fn clicked(&self) -> Option<SceneObjectProps> {
        let mut scene = self.scene.lock();
        scene.get_object(self.clicked).map(|obj| obj.props())
    }
}
