#![allow(dead_code)]
use ui::CaditUi;

mod error;
mod render;
mod ui;

fn main() {
    CaditUi::run();
}
