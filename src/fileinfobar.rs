use gtk::prelude::*;
use gtk::{InfoBar};

pub struct makeInfoBar{
	
}
impl makeInfoBar{	
	
	   pub fn infoBar_config()-> InfoBar{ 
			 let infobar = InfoBar::builder()
			.show_close_button(true)        
            .build();
            
            
            infobar
		}
}
