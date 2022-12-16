use std::{sync::Arc, time::Duration};

use eframe::{
    egui::{self, PointerButton},
    egui_glow,
    epaint::{mutex::Mutex, PaintCallback, Pos2},
};
use three_d::{Deg, InnerSpace, Quaternion, Rad, Rotation3, Vector3};

use crate::ui::{math::AnimatedValue, GlowContext};

use super::scene::{ColorId, ColorIdSource, Scene, SceneObjectProps};

const ROTATION_SENSITIVITY: f32 = 0.007;

#[derive(Debug, Clone, Copy)]
pub enum CameraAngle {
    // Faces
    Front,
    Right,
    Back,
    Left,
    Top,
    Bottom,

    // Edges
    FrontRight,
    BackRight,
    BackLeft,
    FrontLeft,
    FrontTop,
    BackTop,
    FrontBottom,
    BackBottom,
    RightTop,
    LeftTop,
    RightBottom,
    LeftBottom,

    // Corners
    FrontRightTop,
    BackRightTop,
    BackLeftTop,
    FrontLeftTop,
    FrontRightBottom,
    BackRightBottom,
    BackLeftBottom,
    FrontLeftBottom,
}
impl CameraAngle {
    pub fn from_name(name: &str) -> Option<Self> {
        match &*name {
            "Front" => Some(Self::Front),
            "Right" => Some(Self::Right),
            "Back" => Some(Self::Back),
            "Left" => Some(Self::Left),
            "Top" => Some(Self::Top),
            "Bottom" => Some(Self::Bottom),
            "FrontRight" => Some(Self::FrontRight),
            "BackRight" => Some(Self::BackRight),
            "BackLeft" => Some(Self::BackLeft),
            "FrontLeft" => Some(Self::FrontLeft),
            "FrontTop" => Some(Self::FrontTop),
            "BackTop" => Some(Self::BackTop),
            "FrontBottom" => Some(Self::FrontBottom),
            "BackBottom" => Some(Self::BackBottom),
            "RightTop" => Some(Self::RightTop),
            "LeftTop" => Some(Self::LeftTop),
            "RightBottom" => Some(Self::RightBottom),
            "LeftBottom" => Some(Self::LeftBottom),
            "FrontRightTop" => Some(Self::FrontRightTop),
            "BackRightTop" => Some(Self::BackRightTop),
            "BackLeftTop" => Some(Self::BackLeftTop),
            "FrontLeftTop" => Some(Self::FrontLeftTop),
            "FrontRightBottom" => Some(Self::FrontRightBottom),
            "BackRightBottom" => Some(Self::BackRightBottom),
            "BackLeftBottom" => Some(Self::BackLeftBottom),
            "FrontLeftBottom" => Some(Self::FrontLeftBottom),
            _ => None,
        }
    }

    pub fn get_rotation(&self) -> Quaternion<f32> {
        let x: i32 = match self {
            // Faces
            CameraAngle::Front => 0,
            CameraAngle::Right => 0,
            CameraAngle::Back => 0,
            CameraAngle::Left => 0,
            CameraAngle::Top => 270,
            CameraAngle::Bottom => 90,

            // Edges
            CameraAngle::FrontRight => 0,
            CameraAngle::BackRight => 0,
            CameraAngle::BackLeft => 0,
            CameraAngle::FrontLeft => 0,
            CameraAngle::FrontTop => -45,
            CameraAngle::BackTop => -45,
            CameraAngle::FrontBottom => 45,
            CameraAngle::BackBottom => 45,
            CameraAngle::RightTop => -45,
            CameraAngle::LeftTop => -45,
            CameraAngle::RightBottom => 45,
            CameraAngle::LeftBottom => 45,

            // Corners
            CameraAngle::FrontRightTop => -45,
            CameraAngle::BackRightTop => -45,
            CameraAngle::BackLeftTop => -45,
            CameraAngle::FrontLeftTop => -45,
            CameraAngle::FrontRightBottom => 45,
            CameraAngle::BackRightBottom => 45,
            CameraAngle::BackLeftBottom => 45,
            CameraAngle::FrontLeftBottom => 45,
        };

        let y: i32 = match self {
            // Faces
            CameraAngle::Front => 0,
            CameraAngle::Right => -90,
            CameraAngle::Back => 180,
            CameraAngle::Left => 90,
            CameraAngle::Top => 0,
            CameraAngle::Bottom => 0,

            // Edges
            CameraAngle::FrontRight => -45,
            CameraAngle::BackRight => -135,
            CameraAngle::BackLeft => 135,
            CameraAngle::FrontLeft => 45,
            CameraAngle::FrontTop => 0,
            CameraAngle::BackTop => 180,
            CameraAngle::FrontBottom => 0,
            CameraAngle::BackBottom => 180,
            CameraAngle::RightTop => -90,
            CameraAngle::LeftTop => 90,
            CameraAngle::RightBottom => -90,
            CameraAngle::LeftBottom => 90,

            // Corners
            CameraAngle::FrontRightTop => -45,
            CameraAngle::BackRightTop => -135,
            CameraAngle::BackLeftTop => 135,
            CameraAngle::FrontLeftTop => 45,
            CameraAngle::FrontRightBottom => -45,
            CameraAngle::BackRightBottom => -135,
            CameraAngle::BackLeftBottom => 135,
            CameraAngle::FrontLeftBottom => 45,
        };

        Quaternion::from_angle_x(Deg(x as f32)) * Quaternion::from_angle_y(Deg(y as f32))
    }
}

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
    rotation: AnimatedValue<Quaternion<f32>>,
    scene: Arc<Mutex<Scene>>,
    scene_rect: egui::Rect,
    pointer_buttons_down: Vec<PointerButtonDown>,
    clicked: Option<ColorId>,
}
impl ObjectScene {
    pub fn new(gl: GlowContext) -> Self {
        let mut id_source = ColorIdSource::new();

        // Interpolate quaternions:
        /*
        let a = CameraPosition::FrontLeft.get_rotation();
        let b = CameraPosition::Front.get_rotation();
        let c = (a + b) / 2.0;
        */

        Self {
            //rotation: Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), Rad(0.0)),
            rotation: AnimatedValue::new(CameraAngle::FrontRightTop.get_rotation()),
            scene: Arc::new(Mutex::new(Scene::new(gl.clone(), &mut id_source))),
            id_source,
            scene_rect: egui::Rect {
                min: (0.0, 0.0).into(),
                max: (0.0, 0.0).into(),
            },
            pointer_buttons_down: Vec::new(),
            clicked: None,
        }
    }

    fn ui_pos_to_fbo_pos(&self, ui: &egui::Ui, ui_pos: Pos2) -> Pos2 {
        let pix_per_pt = ui.input().pixels_per_point;
        let x = (ui_pos.x - self.scene_rect.min.x) * pix_per_pt;
        let y = (self.scene_rect.max.y - ui_pos.y) * pix_per_pt;
        Pos2 { x, y }
    }

    fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation.set_immediate(rotation);
    }

    fn animate_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation
            .push_swing(rotation, Duration::from_millis(500));
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        self.clicked = None;

        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());

            self.scene_rect = rect;

            let dx = response.drag_delta().x;
            let dy = response.drag_delta().y;

            if dy != 0.0 || dx != 0.0 {
                let rotation = self.rotation.value();
                self.rotation.set_immediate(
                    Quaternion::from_axis_angle(
                        Vector3::new(-dy, dx, 0.0).normalize(),
                        Rad(Vector3::new(dx, dy, 0.0).magnitude() * ROTATION_SENSITIVITY),
                    ) * rotation,
                );
            }

            let scene = self.scene.clone();

            let rotation = self.rotation.value();

            let paint_callback = PaintCallback {
                rect,
                callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                    let mut scene = scene.lock();
                    let context = &scene.context();
                    let frame_input = crate::render::FrameInput::new(context, &info, painter);
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

        if self.rotation.has_queued() {
            ui.ctx().request_repaint();
        }
    }

    fn clicked(&self) -> Option<SceneObjectProps> {
        let mut scene = self.scene.lock();
        scene.get_object(self.clicked).map(|obj| obj.props())
    }
}
