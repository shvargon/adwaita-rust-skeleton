mod application;
mod config;

use adw::prelude::ApplicationExtManual;
use application::Application;

fn main() {
    Application::new().run();
}
