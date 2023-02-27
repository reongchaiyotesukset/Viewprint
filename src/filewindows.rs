extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application,Window, ApplicationWindow};
use gtk::builders::ApplicationBuilder;

pub struct Makewindow{
	
}
impl Makewindow{	
	
	pub fn window_config(app :&gtk::Application)-> gtk::ApplicationWindow
	{
		let window = ApplicationWindow::builder()
		.application(app)
		.default_width(1280)
		.default_height(768)
		.build();
		   window
	}
		
}


