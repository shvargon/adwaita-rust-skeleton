use adw::prelude::*;
use adw::ApplicationWindow;
use gtk::{gio, glib};
use gtk::{Box, Button, Label, Orientation};
use crate::config::APP_ID;
mod imp {
    use super::*;
    use adw::subclass::prelude::*;
    use adw::HeaderBar;

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

            let window = ApplicationWindow::builder()
                .application(app)
                .title("Rust GTK4 App")
                .default_width(300)
                .default_height(200)
                .build();


            let container = Box::new(Orientation::Vertical, 10);

            let button = Button::with_label("Press me!");
            button.connect_clicked(|_| {
                println!("Button clicked!");
            });

            let label = Label::new(Some("Hello is adwaita app"));

            container.append(&HeaderBar::new());
            container.append(&label);
            container.append(&button);

            window.set_content(Some(&container));
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