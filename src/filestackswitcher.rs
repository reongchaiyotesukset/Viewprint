use gtk::prelude::*;
use gtk::{Stack,StackSwitcher};

 pub  struct makeStackSwitcher{
	
}
impl makeStackSwitcher{	
               	
   pub   fn StackSwitcher_config()-> StackSwitcher{ 
        let StackSwitcher = StackSwitcher::builder()
            .build();
            
            StackSwitcher
		}
	pub   fn Stack_config()-> Stack{ 
        let stack = Stack::builder()
            .build();
            
            stack
		}
}
