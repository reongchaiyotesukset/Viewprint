use gtk::prelude::*;
use gtk::{HeaderBar};

pub struct makeHeaderBar{
	
}

impl makeHeaderBar{	
	
	   pub fn headerBar_config()-> HeaderBar{ 
		  
        
        let header = HeaderBar::new();
        
		header.set_show_close_button(true);
        header.set_title(Some("View Print")); 
		header.set_subtitle(Some("Version 0.0.1"));  
    
		      header
		    
		}
}

