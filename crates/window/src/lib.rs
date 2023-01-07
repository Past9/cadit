use egui_winit_vulkano::Gui;
use vulkano::{
    device::{DeviceExtensions, Features},
    format::Format,
};
use vulkano_util::{
    context::{VulkanoConfig, VulkanoContext},
    renderer::VulkanoWindowRenderer,
    window::VulkanoWindows,
};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
};

pub use vulkano_util::window::{WindowDescriptor, WindowMode, WindowResizeConstraints};

pub const IMAGE_FORMAT: Format = Format::B8G8R8A8_SRGB;

pub trait Window {
    fn on_event(
        &mut self,
        event: Event<()>,
        control_flow: &mut ControlFlow,
        renderer: &mut VulkanoWindowRenderer,
        gui: &mut Gui,
    );
}

pub fn run_window<W: Window + 'static>(mut window: W, desc: &WindowDescriptor) -> ! {
    let event_loop = EventLoop::new();
    let context = VulkanoContext::new(VulkanoConfig {
        device_features: Features {
            dynamic_rendering: true,
            sample_rate_shading: true,
            wide_lines: true,
            rectangular_lines: true,
            ..Default::default()
        },
        device_extensions: DeviceExtensions {
            khr_push_descriptor: true,
            khr_swapchain: true,
            ext_line_rasterization: true,
            ..Default::default()
        },
        ..Default::default()
    });

    let mut windows = VulkanoWindows::default();
    windows.create_window(&event_loop, &context, desc, |ci| {
        ci.image_format = Some(IMAGE_FORMAT);
    });

    let mut gui = {
        let renderer = windows.get_primary_renderer_mut().unwrap();

        Gui::new(
            &event_loop,
            renderer.surface(),
            Some(IMAGE_FORMAT),
            renderer.graphics_queue(),
            false,
        )
    };

    event_loop.run(move |event, _, control_flow| {
        let renderer = windows.get_primary_renderer_mut().unwrap();
        window.on_event(event, control_flow, renderer, &mut gui);
    })
}
