extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
use gtk::{Application,Window, ApplicationWindow};
use crate::filelabel::makeLabel;

//add file control
mod filewindows;
mod filebutton;
mod filelayout;
//mod fileentry;
mod filecombobox;
mod filelabel;	
mod filegrid;
mod fileinfobar;
mod filesql;


fn main() {
	
    let app = Application::builder()
        .application_id("Application-testgtk")
        .build();
app.connect_activate(|app| {
	 //add  control 
        let window = filewindows::Makewindow::window_config(app);
        //let text = fileentry::makeEntry::enrty_config();
        let button = filebutton::makeButton::button_config();
        let combo = filecombobox::makeComboBox::combo_config();
        let layout = filelayout::makeLayout::layout_config();
        let label1 = filelabel::makeLabel::label_config_name();
        let label2 = filelabel::makeLabel::label_config_size();
        let grid = filegrid::makeGrid::grid_config();
        let infobar = fileinfobar::makeInfoBar::infoBar_config();
        
        
     
	infobar.content_area().add(&label1);
	infobar.content_area().add(&label2);
	infobar.add_button("Close", gtk::ResponseType::Close.into());                    
	grid.add(&infobar);
	
	
	//show window show all 
	window.add(&grid);
    window.show_all();
    });
    app.run();

}
