#![windows_subsystem = "console"] // 隐藏windows console

use log::error;

mod gui;
use gui::Gui;

pub fn main(){
    // Show the GUI
    if let Err(err) = Gui::start() {
        error!("{err}")
    }
}