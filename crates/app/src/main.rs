#![feature(drain_filter)]

use cadit::ui::CaditUi;

mod ui;

fn main() {
    CaditUi::run();
}
