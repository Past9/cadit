#![allow(dead_code)]
#![feature(drain_filter)]
use ui::CaditUi;

mod error;
mod render;
mod ui;

fn main() {
    CaditUi::run();
}
