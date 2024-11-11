use gtk::{gio, glib};
use crate::config::APP_ID;

mod imp {
    use super::*;
    use crate::window::Window;
    use adw::subclass::prelude::*;

    #[derive(Default, Debug)]
    pub struct Application;

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "MyApp";
        type Type = super::Application;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for Application {}

    impl ApplicationImpl for Application {
        fn activate(&self) {
            let app: &super::Application = &self.obj();
            let window = Window::new(app);
            window.present();
        }
    }
    impl GtkApplicationImpl for Application {}
    impl AdwApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application;
}

impl Application {
    pub fn new() -> Self {
        glib::Object::builder::<Application>()
            .property("application-id", APP_ID)
            .property("flags", gio::ApplicationFlags::HANDLES_OPEN)
            .build()
    }
}