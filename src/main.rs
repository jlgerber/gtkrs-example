//! how do we go about creating a mainwindow? This is a cannonical structure
//! for gtk applicatoons
//!
use std::env;

use gio::{ApplicationExt, ApplicationExtManual, ApplicationFlags};
use gtk::{
    Application,
    ApplicationWindow,
    WidgetExt,
    GtkWindowExt,
};


fn main() {
    let application = Application::new("com.github.jlgerber.myexample", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(|application| {
        let window = ApplicationWindow::new(application);
        window.set_title("Foobar");
        window.show();
    });
    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}

