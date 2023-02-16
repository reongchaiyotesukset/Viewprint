use gtk::prelude::*;
use gtk::{ComboBox};

pub struct makeComboBox{
	
}
impl makeComboBox{	
	
	  pub fn combo_changed(combo: &gtk::ComboBoxText) {
            let text = combo.active_text().expect("No active text");
            println!("Selected: {}", text);
         }
         
	   pub fn combo_config()-> gtk::ComboBoxText{ 
		  
         let  combo = gtk::ComboBoxText::builder()
                     .margin(100)
					 .margin_bottom(100)
					 .margin_top(100)
					 .margin_start(25)
					 .margin_end(100)
					 .build();
					 
				 combo.append_text("Option 1");
				 combo.append_text("Option 2");
                 combo.append_text("Option 3");               
		      combo
		    
		}
}
