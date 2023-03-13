extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
use gtk::{Application,Window, ApplicationWindow,};
use gtk::Label;
use gtk::Button;
use gtk::Grid;
use gtk::Box;
use mysql::from_row;
use mysql::Row;
use mysql::prelude::Queryable;

use std::thread;
use std::time::Duration;
use std::time::Instant;
use std::thread::sleep;
mod filesql;


fn main() {
let app = Application::builder()
        .application_id("Application-testgtk")
        .build();
        
app.connect_activate(|app| {
	
	let window = ApplicationWindow::builder()
		.application(app)
		.default_width(380)
		.default_height(368)
		.build();
		
	let button1 = Button::builder()
           .build();
			button1.set_label("Query Job");

	let button2 = Button::builder()
           .build();
			button2.set_label("Button!!!"); 
			
	let jobnamelabel1 = Label::new(Some("Hello World1"));
	let jobnamelabel2 = Label::new(Some("Hello World2"));
	let jobnamelabel3 = Label::new(Some("Hello World3"));	

	let gridmain = Grid::new();
		gridmain.attach(&button1, 0, 0 , 1, 1);




		button1.connect_clicked(move|gridmain| {
			let griddata = Grid::new();
			
               let resultlabel = ResultData();
       
		});
		        


   
	window.add(&gridmain);
	window.show_all();
});
    app.run();
}
fn ResultData()
{
	  let sql_queryinfobar = select_return_Vec();	
	    let label1 = Label::new(Some("test")); 
	for (id, jobname,jobsize,image,typename,activename) in &sql_queryinfobar 
	 {
		 
		   let jobnameformat = &format!("{}{}{}","<span font_desc='Sans 25'>",jobname,"</span>");
		    label1.set_markup(&jobnameformat);
         println!("{:?}",id);
	  }
	  
}


fn select_return_Vec()-> Vec<(i32, String,String,String,String,String)>
{	
		
				
	        let conn =  filesql::mySql::connect_server();
                                                    
           let query = "SELECT  job.id,
              job.jobname,job.jobsize,job.image,jobtype.typename,
               activeuse.activename FROM job  LEFT JOIN jobtype
              on job.jobtype_id  = jobtype.jobtype_id  LEFT
              JOIN activeuse  on job.active_id   =
              activeuse.active_id";
						  
        let result  = conn.expect("There is a problem with querying data.").query_map(query, |row: Row| {
	  
        let (id, jobname,jobsize,image,typename,activename) = from_row(row);
        (id, jobname,jobsize,image,typename,activename)
    }).unwrap();

	    result
}
