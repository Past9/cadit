use cgmath::Quaternion;

pub mod assembly;
pub mod part;

pub trait Editor {
    fn title(&self) -> String;
    fn show(&mut self, ui: &mut eframe::egui::Ui);
    //fn clicked(&self) -> Option<SceneObjectProps>;
    fn set_rotation(&mut self, rotation: Quaternion<f32>);
    //fn animate_rotation(&mut self, rotation: Quaternion<f32>);
}
