use gtk::prelude::*;
use gtk::{Entry,EntryCompletion};

pub struct makeEntry{
	
}
impl makeEntry{	
	
		pub fn create_entry_completion() -> gtk::EntryCompletion {
			let completion = gtk::EntryCompletion::new();
			
			let liststore = gtk::ListStore::new(&[glib::Type::STRING]);
			completion.set_model(Some(&liststore));
			completion.set_text_column(0);
		    let var_u32: Option<u32> = Some(0);
			liststore.insert_with_values(Some(0), &[(0, &"Computer")]);
            liststore.insert_with_values(Some(0), &[(0, &"Ram")]);
            liststore.insert_with_values(Some(0), &[(0, &"Mainboard")]);
            liststore.insert_with_values(Some(0), &[(0, &"CPU")]);
            liststore.insert_with_values(Some(0), &[(0, &"VGA Card")]);
            liststore.insert_with_values(Some(0), &[(0, &"USB")]);
		completion
     }
     
     /*
       pub fn create_entry_completion() -> gtk::EntryCompletion{
		    let liststore = gtk::ListStore::new(&[glib::Type::STRING]);
			let completion = gtk::EntryCompletion::builder()			
							.model(&liststore)
							.text_column(0)
							.inline_completion(true)
							.inline_selection(true)
							.popup_completion(true)
							.popup_set_width(true)
                            .popup_single_match(true)
							.build();
						
			let Apple: Option<u32> = Some(0);
			liststore.insert_with_values(Apple, &[(0, &"Apple")]);
             println!("{}",completion);
		     completion
       }
     */
	   pub fn enrty_config()-> Entry{ 
        let text = Entry::builder()
            .placeholder_text("input")
            .can_default(true)
            .can_focus(true)
            .completion(&Self::create_entry_completion())
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();
		    text
		    
		}
}
