use adw::prelude::AdwApplicationWindowExt;
use adw::ApplicationWindow;
use gtk::prelude::*;
use gtk::{Box, Button, HeaderBar, Label, Orientation};

// Структура для окна
pub struct Window {
    window: ApplicationWindow,
}

impl Window {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        // Создаем окно
        let window = ApplicationWindow::builder()
            .application(application)
            .title("My Application")
            .default_width(300)
            .default_height(200)
            .build();

        let container = Box::new(Orientation::Vertical, 10);

        let label = Label::new(Some("Hello is adwaita app"));

        let button = Button::with_label("Press me!");
        button.connect_clicked(|_| {
            println!("Button clicked!");
        });

        container.append(&HeaderBar::new());
        container.append(&label);
        container.append(&button);

        window.set_content(Some(&container));

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
