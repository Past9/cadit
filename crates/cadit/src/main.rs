#![allow(dead_code)]
use ui::CaditUi;
use window::{run_window, WindowDescriptor};

mod error;
mod ui;

fn main() {
    run_window(
        CaditUi::new(),
        &WindowDescriptor {
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        },
    )
}
