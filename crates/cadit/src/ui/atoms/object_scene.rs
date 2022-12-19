use super::scene::{ColorId, ColorIdSource, Scene, SceneObjectProps};
use crate::ui::GlowContext;
use eframe::{
    egui::{self, PointerButton},
    egui_glow,
    epaint::{
        mutex::Mutex, pos2, vec2, CircleShape, PaintCallback, Pos2, Rect, Rgba, Shape, Stroke, Vec2,
    },
};
use std::{fmt::Pointer, sync::Arc};
use three_d::{
    Blend, BlendEquationType, BlendMultiplierType, InnerSpace, Quaternion, Rad, RenderStates,
    Rotation3, Vector2, Vector3,
};

const ROTATION_SENSITIVITY: f32 = 0.007;
const PAN_SENSITIVITY: f32 = 0.01;

#[derive(Clone, Debug)]
pub struct PointerButtonDown {
    pos: Pos2,
    button: PointerButton,
    start_time: std::time::Instant,
    start_position: Pos2,
    last_position: Pos2,
    modifiers: eframe::egui::Modifiers,
    start_scene_object: Option<ColorId>,
}

pub struct ObjectScene {
    scene: Arc<Mutex<Scene>>,
    scene_rect: egui::Rect,
    id_source: ColorIdSource,
    rotation: Quaternion<f32>,
    position: Vector2<f32>,
    pointer_buttons_down: Vec<PointerButtonDown>,
    clicked: Option<ColorId>,
    rotated: bool,
    allow_manual_rotate: bool,
    allow_manual_pan: bool,
}
impl ObjectScene {
    pub fn new(
        gl: GlowContext,
        rotation: Quaternion<f32>,
        position: Vector2<f32>,
        allow_manual_rotate: bool,
        allow_manual_pan: bool,
    ) -> Self {
        let mut id_source = ColorIdSource::new();

        Self {
            scene: Arc::new(Mutex::new(Scene::new(gl.clone(), &mut id_source))),
            scene_rect: egui::Rect {
                min: (0.0, 0.0).into(),
                max: (0.0, 0.0).into(),
            },
            id_source,
            rotation,
            position,
            pointer_buttons_down: Vec::new(),
            clicked: None,
            rotated: false,
            allow_manual_rotate,
            allow_manual_pan,
        }
    }

    pub fn rotated(&self) -> bool {
        self.rotated
    }

    fn ui_pos_to_fbo_pos(&self, ui: &egui::Ui, ui_pos: Pos2) -> Pos2 {
        let pix_per_pt = ui.input().pixels_per_point;
        let x = (ui_pos.x - self.scene_rect.min.x) * pix_per_pt;
        let y = (self.scene_rect.max.y - ui_pos.y) * pix_per_pt;
        Pos2 { x, y }
    }

    pub fn rotation(&mut self) -> Quaternion<f32> {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }

    pub fn rect(&self) -> Rect {
        self.scene_rect
    }

    pub fn middle_drag(&self) -> Option<&PointerButtonDown> {
        self.pointer_buttons_down
            .iter()
            .find(|b| b.button == PointerButton::Middle)
    }

    pub fn secondary_drag(&self) -> Option<&PointerButtonDown> {
        self.pointer_buttons_down
            .iter()
            .find(|b| b.button == PointerButton::Secondary)
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.clicked = None;
        self.rotated = false;

        egui::Frame::canvas(ui.style())
            .fill(Rgba::TRANSPARENT.into())
            .show(ui, |ui| {
                // Update and track the scene dimensions
                let (rect, _response) =
                    ui.allocate_at_least(ui.available_size(), egui::Sense::drag());
                self.scene_rect = rect;

                // Handle mouse events
                let mut scene = self.scene.lock();
                for event in ui.input().events.iter() {
                    match event {
                        egui::Event::PointerMoved(pos) => {
                            // If the mouse moved over a scene object, flag that object as hovered
                            let obj_id = scene.read_color_id(self.ui_pos_to_fbo_pos(ui, *pos));
                            scene.hover_object(obj_id);

                            // If manual rotation is enabled
                            if self.allow_manual_rotate {
                                // Rotate the model on secondary mouse button drag
                                if let Some(rotation_drag) = self.secondary_drag() {
                                    let Vec2 { x: dx, y: dy } = *pos - rotation_drag.last_position;

                                    println!(
                                        "start {} {}, {} {}",
                                        rotation_drag.start_position.x,
                                        rotation_drag.start_position.y,
                                        rect.left_top().x,
                                        rect.left_top().y
                                    );

                                    if dy != 0.0 || dx != 0.0 {
                                        self.rotation = Quaternion::from_axis_angle(
                                            Vector3::new(-dy, dx, 0.0).normalize(),
                                            Rad(Vector3::new(dx, dy, 0.0).magnitude()
                                                * ROTATION_SENSITIVITY),
                                        ) * self.rotation;
                                        self.rotated = true;
                                    }
                                }
                            }

                            // If manual panning is enabled
                            if self.allow_manual_pan {
                                // Pan the model on middle button drag
                                if let Some(pan_drag) = self.middle_drag() {
                                    let Vec2 { x: dx, y: dy } = *pos - pan_drag.last_position;

                                    if dy != 0.0 || dx != 0.0 {
                                        self.position += Vector2::new(dx, dy) * PAN_SENSITIVITY;
                                    }
                                }
                            }

                            // Update the last_position of all the currently pressed mouse buttons
                            self.pointer_buttons_down
                                .iter_mut()
                                .for_each(|b| b.last_position = *pos);
                        }
                        egui::Event::PointerButton {
                            pos,
                            button,
                            pressed,
                            modifiers,
                        } => {
                            // Check if there's a scene object at the current mouse position.
                            let obj_id = scene.read_color_id(self.ui_pos_to_fbo_pos(ui, *pos));

                            if *pressed {
                                if self.scene_rect.contains(*pos) {
                                    // If a button was just pressed, update the list of currently-pressed
                                    // mouse buttons

                                    // Remove the button that was just pressed, in case it's still
                                    // somehow in the list (it should have been removed last time the
                                    // button was released)
                                    self.pointer_buttons_down
                                        .retain(|down| down.button != *button);

                                    // Now re-add the button that was just pressed with appropriate starting
                                    // properties
                                    self.pointer_buttons_down.push(PointerButtonDown {
                                        pos: *pos,
                                        button: *button,
                                        start_time: std::time::Instant::now(),
                                        start_position: *pos,
                                        last_position: *pos,
                                        modifiers: modifiers.to_owned(),
                                        start_scene_object: obj_id,
                                    });
                                }
                            } else {
                                if *button == PointerButton::Primary {
                                    if let Some(obj_id) = obj_id {
                                        // If we released the primary button on a scene object,
                                        // check if we also pressed that button on the same object.
                                        // If so, this counts as clicking on the object.
                                        let down = self.pointer_buttons_down.iter().find(|down| {
                                            down.button == PointerButton::Primary
                                                && down.start_scene_object == Some(obj_id)
                                        });

                                        // If we clicked an object...
                                        if let Some(down) = down {
                                            // Determine if the shift key is currently pressed. If so, then
                                            // we'll add the object to the current selection, otherwise we'll
                                            // select the clicked object and deselect everything else.
                                            let shift_select =
                                                down.modifiers.shift && modifiers.shift;

                                            // Do the selection
                                            scene.toggle_select_object(Some(obj_id), !shift_select);

                                            // Set self.clicked to the clicked object so the parent
                                            // can respond if needed.
                                            if scene.get_object(Some(obj_id)).is_some() {
                                                self.clicked = Some(obj_id);
                                            }
                                        }
                                    } else {
                                        // If we released the mouse button over empty space, we deselect everything.
                                        if !modifiers.shift {
                                            scene.deselect_all_objects();
                                        }
                                    }
                                }

                                // Remove the button that was just released from the list of
                                // currently-pressed buttons
                                self.pointer_buttons_down
                                    .retain(|down| down.button != *button);
                            }
                        }
                        _ => {}
                    }
                }

                // Clone stuff for the paint callback
                let scene = self.scene.clone();
                let rotation = self.rotation;
                let position = self.position;

                // Create the paint callback
                let paint_callback = PaintCallback {
                    rect,
                    callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                        let mut scene = scene.lock();
                        let context = scene.context();
                        let frame_input = crate::render::FrameInput::new(context, &info, painter);
                        scene.render(frame_input, rotation, position);
                    })),
                };

                // Queue the scene for painting
                ui.painter().add(paint_callback);
            });
    }

    pub fn clicked(&self) -> Option<SceneObjectProps> {
        let mut scene = self.scene.lock();
        scene.get_object(self.clicked).map(|obj| obj.props())
    }
}
