use gtk::prelude::*;
use gtk::{Label};
use crate::filesql::mySql;

pub struct makeLabel {
	
}

impl makeLabel {	

	 pub fn label_config_1()-> Label
	   { 
	        //mySql::selectsql();
	        //let getdata = mySql::selectreturn();
	        
	        
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true)  
             .margin(0) 
             .margin_top(30) 
             .margin_start(30)       
             .margin_end(30)         
            .build();	
            label.set_markup("<span font_desc=\"Sans 19\">มีงานใหม่เข้ามา </span>"); 
      
		    label        
		}
	   pub fn label_config_2()-> Label
	   { 
	        //mySql::selectsql();
	        //let getdata = mySql::selectreturn();
	        
	        
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true) 
             .margin(0) 
             .margin_top(30) 
             .margin_start(240)       
             .margin_end(30)       
            .build();
            label.set_markup("<span font_desc=\"Sans 30\">0</span>");			
            
		    label        
		}
	  pub fn label_config_3()-> Label
	   { 
	        //mySql::selectsql();
	        //let getdata = mySql::selectreturn();
	        
	        
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true) 
             .margin(0) 
             .margin_top(30) 
             .margin_start(300)       
             .margin_end(30)       
            .build();
            label.set_markup("<span font_desc=\"Sans 19\">งาน</span>");			
            
		    label        
		}
	   pub fn label_config_4()-> Label
	   { 
	        //mySql::selectsql();
	        //let getdata = mySql::selectreturn();
	        
	        
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true) 
             .margin(0) 
             .margin_top(30) 
             .margin_start(450)       
             .margin_end(30)       
            .build();
             label.set_markup("<span font_desc=\"Sans 19\">รับงาน:</span>");				
            
		    label        
		}
	 pub fn label_config_5()-> Label
	   { 
	        //mySql::selectsql();
	        //let getdata = mySql::selectreturn();
	        
	        
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true) 
             .margin(0) 
             .margin_top(30) 
             .margin_start(600)       
             .margin_end(30)       
            .build();
            label.set_markup("<span font_desc=\"Sans 30\">0</span>");			
            
		    label        
		}
		pub fn label_config_6()-> Label
	   { 
	        //mySql::selectsql();
	        //let getdata = mySql::selectreturn();
	        
	        
            let label = Label::builder()                 
            .can_default(true)
            .can_focus(true) 
             .margin(0) 
             .margin_top(30) 
             .margin_start(650)       
             .margin_end(30)       
            .build();
            label.set_markup("<span font_desc=\"Sans 19\">งาน</span>");			
            
		    label        
		}
		
}
