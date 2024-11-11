mod application;
mod config;
mod window;

use adw::prelude::ApplicationExtManual;
use application::Application;

fn main() {
    Application::new().run();
}
