extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
use gtk::{Application,Window, ApplicationWindow};

mod filewindows;
mod filesql;
mod folder_infobardetial;
mod fileheaderbar;

use crate::folder_infobardetial::makeInfoBarDetial;
use crate::filesql::mySql;

fn main() {
	
    let app = Application::builder()
        .application_id("Application-testgtk")
        .build();
        
app.connect_activate(|app| {

let window = filewindows::Makewindow::window_config(app);
let header =  fileheaderbar::makeHeaderBar::headerBar_config();


let infobarDetial = makeInfoBarDetial::infobarDetial_config();



window.set_titlebar(Some(&header));
window.add(&infobarDetial);
	 
    window.show_all();
    });
    app.run();

}
