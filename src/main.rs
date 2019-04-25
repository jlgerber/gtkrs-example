
// bring in the environment, which is used in the application's run method
use std::env;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use gio::{
    ApplicationExt,
    ApplicationExtManual,
     ApplicationFlags,
     ActionMapExt,
    FileExt,
    };

use gtk::{
    Application,
    ApplicationWindow,
    GtkApplicationExt,
    Builder,
    BuilderExt,
    DialogExt,
    FileChooserExt,
    GtkWindowExt,
    NativeDialogExt,
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
pub fn add_actions(app: &Application, window: &ApplicationWindow, sender: Sender<std::path::PathBuf>) {
    let open = gio::SimpleAction::new("open", None);
    app.add_action(&open);

    open.connect_activate(clone!(window => move |_, _| {

        let dialog = gtk::FileChooserNative::new(
            "Open File",
            Some(&window),
            gtk::FileChooserAction::Open,
            "Ok",
            "Cancel"
        );
        let result = dialog.run();
        if result == -3 {
            if let Some(fname) = dialog.get_file() {
                //println!("the file is {:?}", fname.get_path().unwrap());
                let _ = sender.send(fname.get_path().unwrap());
            }
        } else {
            println!("Nope");
        }
    }));
}

fn main() {
    let application = Application::new(
        "com.github.problem.open_segfaults",
        ApplicationFlags::empty()
    ).expect("Application initialization failed");

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

        let (sender, receiver) = channel();
        let (sender2, receiver2) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        add_actions(&application, &window, sender);

        // handle file open requests
        // do some work.
        std::thread::spawn(move || {
            let msg = receiver.recv().unwrap();
            let _ = sender2.send("loading".to_string());
            for _ in 0..10 {
                let _ = sender2.send(".".to_string());
                //print!(".");
                std::thread::sleep(std::time::Duration::from_secs(1));
            }

            let _ = sender2.send(msg.into_os_string().into_string().unwrap());
        });


        receiver2.attach(None, move |msg| {
            println!("received {:?}", msg);
            // Returning false here would close the receiver
            // and have senders fail
            glib::Continue(true)
        });

        window.show();
    });

    application.connect_activate(|_| {});
    application.run(&env::args().collect::<Vec<_>>());
}

