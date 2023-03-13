extern crate gtk;
use gtk::prelude::*;
use gtk::Button;
use gtk::Grid;
use gtk::Label;
use gtk::RadioButton;
use gtk::{Application,Window, ApplicationWindow};
fn buttonMake()->Button
{
	 let returnbutton = gtk::Button::new();
	 returnbutton
}

fn main() {
	
    let app = Application::builder()
        .application_id("Application-testgtk")
        .build();
        
app.connect_activate(|app| {
		let window = ApplicationWindow::builder()
		.application(app)
		.default_width(300)
		.default_height(300)
		.build();
		
		let grid = Grid::new();
		let button = Button::new();
	
     let lable1 = Label::new(Some("NO LABEL"));
	 let radio1 = RadioButton::new();
	     radio1.set_label("Man"); 

	 let radio2 = RadioButton::new();	
	     radio2.join_group(Some(&radio1));	
	     radio2.set_label("Woman"); 
	     
	    
		
	    
	    grid.attach(&radio1, 0, 0 , 1, 1);
	    grid.attach(&radio2, 0, 1 , 1, 1);
	    grid.attach(&lable1, 0, 2 , 1, 1);
	    
	
	    radio1.connect_toggled(move|radio1| {
	        
	        if radio1.is_active() {
				lable1.set_label("Man");
			}
             if radio2.is_active() {
				lable1.set_label("Woman");
			} 
        });	               
window.add(&grid);             
window.show_all();
});
    app.run();

}
