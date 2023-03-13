use gtk::prelude::*;
use gtk::{HeaderBar,Button};

use mysql::from_row;
use mysql::Row;
use crate::mySql;
use mysql::prelude::Queryable;

 use gtk::Label;
pub struct makeHeaderBar{
	
}

impl makeHeaderBar{	
fn ResultData()
{
	  let sql_queryinfobar = Self::select_return_Vec();	
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
		
				
	        let conn =  mySql::connect_server();
                                                    
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

	
pub fn headerBar_config()-> HeaderBar{ 
		   
		 let button = Button::new();
		 button.set_label("Refresh");
         let header = HeaderBar::builder()
             .title("View Print")
             .subtitle("Powered by GTK and Rust Version 0.0.1")
             .show_close_button(true)
             .build();
             header.add(&button);
             
			button.connect_clicked(move|gridmain| {
			   //let griddata = Grid::new();
               let resultlabel = Self::ResultData();
       
			});
		      header
		    
		}
}

