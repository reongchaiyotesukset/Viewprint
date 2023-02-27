extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
use gtk::{Application,Window, ApplicationWindow};
mod filewindows;
//mod filegrid;
mod filesql;
mod folder_infobardetial;
use crate::folder_infobardetial::makeInfoBarDetial;
use crate::filesql::mySql;

fn main() {
	
    let app = Application::builder()
        .application_id("Application-testgtk")
        .build();
        
app.connect_activate(|app| {

let window = filewindows::Makewindow::window_config(app);
let infobarDetial = makeInfoBarDetial::infobarDetial_config();

	 window.add(&infobarDetial);
    window.show_all();
    });
    app.run();

}
