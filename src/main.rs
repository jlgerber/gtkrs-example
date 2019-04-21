

// bring in the environment, which is used in the application's run method
use std::env;


use gio::{
    ApplicationExt,
    ApplicationExtManual,
     ApplicationFlags,
     // add_action for application
     ActionMapExt
    };
use gtk::{
    // Application provides
    Application,
    ApplicationWindow,
    GtkApplicationExt,
    Builder,
    BuilderExt,
    DialogExt,
    GtkWindowExt,
    WidgetExt,
};


const MENUS_STR: &'static str = r#"
<?xml version="1.0"?>
<interface>
  <menu id="appmenu">
    <section>
      <item>
        <attribute name="label" translatable="yes">Open</attribute>
        <attribute name="action">app.open</attribute>
      </item>
    </section>
   </menu>
</interface>
"#;

/// make moving clones into closures more convenient
/// since we are cloning all the time when defining closures for signals / slots
/// this macro makes it a bit simpler
///
/// # Usage
/// ```rust
/// widget.connect_some_event(clone!(foo,bar => move |_|{
///     do my thing
/// }));
/// ```
#[macro_export]
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
        }
    );
}

/// components should not be reference counted coming out of glade, as they are
/// already tracked
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

/// add actions for application
pub fn add_actions(app: &Application, window: &ApplicationWindow) {
    let open = gio::SimpleAction::new("open", None);
    app.add_action(&open);

    open.connect_activate(clone!(window => move |_, _| {

        let dialog = gtk::FileChooserDialog::with_buttons(
            "Open File",
            Some(&window),
            gtk::FileChooserAction::Open,
            &[ ("Ok",     gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
            ]
        );
        dialog.set_transient_for(Some(&window));
        dialog.run();
        dialog.destroy();
    }));

}

fn main() {
    let application = Application::new("com.github.jlgerber.myexample", ApplicationFlags::empty())
        .expect("Application initialization failed");
    application.connect_startup(|application| {

        let window = ApplicationWindow::new(application);
        window.set_title("Foobar");

        // create a builder
        let builder = gtk::Builder::new();
        // add the xml which defines our menus
        builder.add_from_string(&MENUS_STR).unwrap();
        // take care of the application menu
        let appmenu: gio::MenuModel = builder.get_object("appmenu").unwrap();
        application.set_app_menu(&appmenu);

        add_actions(&application, &window);

        window.show();
    });

    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}

