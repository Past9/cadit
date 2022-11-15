#![feature(drain_filter)]
use ui::CaditUi;

mod error;
mod ui;

fn main() {
    CaditUi::run();
}
