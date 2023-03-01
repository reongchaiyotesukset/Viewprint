use gtk::prelude::*;
use gtk::{HeaderBar};

pub struct makeHeaderBar{
	
}

impl makeHeaderBar{	
	
	   pub fn headerBar_config()-> HeaderBar{ 
		  
        
         let header = HeaderBar::builder()
             .title("View Print")
             .subtitle("Powered by GTK and Rust Version 0.0.1")
             .show_close_button(true)
             .build();
             
    
		      header
		    
		}
}

