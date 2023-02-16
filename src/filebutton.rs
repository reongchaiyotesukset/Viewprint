use gtk::prelude::*;
use gtk::{Button};

pub struct makeButton{
	
}
impl makeButton{	
	
	   pub fn button_config()-> Button{ 
		  
         let button1 = Button::builder()
            .label("Click1")
            .margin(60)
            .margin_bottom(60)
            .margin_top(60)
            .margin_start(60)
            .margin_end(60)
            .build();
             button1.connect_clicked(|_| {
                 eprintln!("Clicked1");
		    });
		      button1
		    
		}
}
