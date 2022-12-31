#![allow(dead_code)]
use egui_winit_vulkano::Gui;
use ui::CaditUi;
use vulkano::device::Features;
use vulkano_util::{
    context::{VulkanoConfig, VulkanoContext},
    window::{VulkanoWindows, WindowDescriptor},
};
use winit::event_loop::EventLoop;

mod error;
mod render;
mod ui;

fn main() {
    let event_loop = EventLoop::new();
    let context = VulkanoContext::new(VulkanoConfig {
        device_features: Features {
            dynamic_rendering: true,
            ..Default::default()
        },
        ..Default::default()
    });
    let mut windows = VulkanoWindows::default();
    windows.create_window(
        &event_loop,
        &context,
        &WindowDescriptor {
            width: 1760.0,
            height: 990.0,
            ..Default::default()
        },
        |ci| ci.image_format = Some(vulkano::format::Format::B8G8R8A8_SRGB),
    );

    let mut gui = {
        let renderer = windows.get_primary_renderer_mut().unwrap();

        Gui::new(
            &event_loop,
            renderer.surface(),
            Some(vulkano::format::Format::B8G8R8A8_SRGB),
            renderer.graphics_queue(),
            false,
        )
    };

    let mut cadit = CaditUi::new();

    event_loop.run(move |event, _, control_flow| {
        let renderer = windows.get_primary_renderer_mut().unwrap();
        cadit.on_event(event, control_flow, renderer, &mut gui);
    })
}
