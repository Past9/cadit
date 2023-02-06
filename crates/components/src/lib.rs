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

pub use egui_winit_vulkano::Gui;
pub use vulkano_util::window::{WindowDescriptor, WindowMode, WindowResizeConstraints};

mod util;

pub mod editors;
pub mod gizmo;
pub mod panes;
pub mod scene;

pub use render::{rgb, rgba, Rgb, Rgba};

pub const IMAGE_FORMAT: Format = Format::B8G8R8A8_SRGB;

pub trait Window {
    fn draw(&mut self, gui: &mut Gui);

    fn on_close(&mut self) -> bool {
        return true;
    }

    fn on_event(
        &mut self,
        _event: &Event<()>,
        _control_flow: &mut ControlFlow,
        _renderer: &mut VulkanoWindowRenderer,
        _gui: &mut Gui,
    ) {
        // Do nothing
    }
}

pub fn run_window<W: Window + 'static>(mut window: W, desc: &WindowDescriptor) -> ! {
    let event_loop = EventLoop::new();
    let context = VulkanoContext::new(VulkanoConfig {
        device_features: Features {
            dynamic_rendering: true,
            sample_rate_shading: true,
            wide_lines: true,
            rectangular_lines: true,
            independent_blend: true,
            ..Default::default()
        },
        device_extensions: DeviceExtensions {
            khr_push_descriptor: true,
            khr_swapchain: true,
            ext_line_rasterization: true,
            ext_blend_operation_advanced: true,
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

        match &event {
            Event::WindowEvent { window_id, event } => {
                if *window_id != renderer.window().id() {
                    return;
                }

                gui.update(&event);

                match event {
                    winit::event::WindowEvent::Resized(_)
                    | winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                        renderer.resize();
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        if window.on_close() {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    _ => (),
                }
            }
            Event::RedrawRequested(window_id) => {
                if *window_id != renderer.window().id() {
                    return;
                }

                window.draw(&mut gui);

                let before_future = renderer.acquire().unwrap();
                let after_future =
                    gui.draw_on_image(before_future, renderer.swapchain_image_view());
                renderer.present(after_future, true);
            }
            Event::MainEventsCleared => {
                renderer.window().request_redraw();
            }
            _ => (),
        };

        window.on_event(&event, control_flow, renderer, &mut gui);
    })
}
