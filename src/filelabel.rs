use gtk::prelude::*;
use gtk::{Label};

pub struct makeLabel{
	
}
impl makeLabel{	
	
	   pub fn label_config()-> Label
	   { 
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true)          
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();

            label.set_markup("<span font_desc=\"Sans 19\">Hello, world!</span>"); 
            
		    label
		    
		    
		}
		
		pub fn label_set_markup()
		{ 
			let label= Self::label_config();
			label.set_markup("<span font_desc=\"Sans 19\">Hello, world!</span>"); 
		}
		
}
