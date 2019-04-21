How do we go about creating a mainwindow? This is a cannonical structure for gtk applicatoons

 There are two main structures which come into play when creating  a gtk application

 # gtk::Application

 The `gtk::Application` is responsible for GTK+ initialization, application uniqueness, and session management; it provides some basic
 scriptability and desktop shell integration by exporting actions and menus and manages a list of toplevel windows whose life-cycle is automatically tied to the life-cycle of your application.

## Components


### Application Menu
The application menu is the leftmost menu on the main application window, or, in the case of some oses (os x for example) rendered on the desktop in the top lefthand corner. It is named after the application and usually has `About` and `Quit`, and maybe `Preferences`. Qtk differentiates between this menu and the application menubar, even though, in most cases , the application menubar will be rendered to the right of the application menu.

The `application` is responsible for registering the application menu via the `set_app_menu` method.

### Menubar
  - set_menubar
### Accellerators
  - set_accels_for_menu
