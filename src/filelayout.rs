use gtk::prelude::*;
use gtk::{Layout};

pub struct makeLayout{
	
}
impl makeLayout{	
	
	   pub fn layout_config()-> Layout{ 
		  
        
        let layout = Layout::builder()
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();
        
		      layout
		    
		}
}

