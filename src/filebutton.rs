use gtk::prelude::*;
use gtk::{Button};

pub struct makeButton{
	
}
//6788
impl makeButton{	
	
	   pub fn button_config()-> Button{ 
		  
         let button1 = Button::builder()
            .label("Click1")
            .build();
		      button1
		    
		}
}
