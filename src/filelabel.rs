use gtk::prelude::*;
use gtk::{Label};
use crate::filesql::mySql;

pub struct makeLabel{
	
}
//fetch
impl makeLabel{	

	   pub fn label_config_name()-> Label
	   { 
	        //mySql::selectsql();
	        //let getdata = mySql::selectreturn();
	        
	        
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true)          
            .build();

             //let text = &format!("{}{}{}","<span font_desc=\"Sans 19\">",getdata,"</span>");
             //label.set_markup(text); 
           
            
		    label
		    
		    
		}
		pub fn label_config_size()-> Label
	   { 
	        //mySql::selectsql();
	        //let getdata = mySql::select_return_String();
	        //let getdata = mySql::select_return_Vec();
	        
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true)          
            .build();

             //let text = &format!("{}{}{}","<span font_desc=\"Sans 19\">",getdata,"</span>");
             //label.set_markup(text); 
           
            
		    label
		    
		    
		}
		
}
