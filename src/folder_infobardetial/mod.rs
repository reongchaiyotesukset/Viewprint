use gtk::prelude::*;
use gtk::{InfoBar,Label,Button,Grid};
use mysql::*;
use mysql::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::filesql::mySql;


#[derive(Debug, PartialEq, Eq)]
pub struct makeInfoBarDetial{
	
}
impl makeInfoBarDetial{	
                       	
fn sleep() {
	
	let interval = Duration::from_secs(15);
	let mut next_time = Instant::now() + interval;
	loop {
		println!("Hello World");
		sleep(next_time - Instant::now());
		next_time += interval;
	}
}                     	
 pub fn infobarDetial_config()-> Grid{ 
	 
		   let conn =  mySql::connect_server();
		   let grid1 = Grid::new();
		   
		   
		   

	//function mysql query 	                                          
	let sql_queryinfobar = Self::select_return_Vec();
		   

	
//loop object and data on mysql
for (id, jobname,jobsize,image,typename,activename) in &sql_queryinfobar 
{
					
		
  				  
 let infobar123 = InfoBar::new();
//make label
let jobname_label = Label::new(Some("NO LABEL1"));
let jobsize_label = Label::new(Some("NO LABEL2"));
let jobtype_label  =  Label::new(Some("NO LABEL3"));
let activeuse_label  =  Label::new(Some("NO LABEL4"));
//make button
let button = Button::new();
////conver format utf8 on mysql
let jobname_format = &format!("{}{}{}","<span font_desc='Sans 25'>",jobname,"</span>");
let jobsize_format = &format!("{}{}{}","<span font_desc='Sans 25'>",jobsize,"</span>");
let typename_format = &format!("{}{}{}","<span font_desc='Sans 25'>",typename,"</span>");
let activeuse_format = &format!("{}{}{}","<span font_desc='Sans 25'>",activename,"</span>");

       //resize  big font 
		jobname_label.set_markup(&jobname_format);
		jobsize_label.set_markup(&jobsize_format);
		jobtype_label.set_markup(&typename_format);
		activeuse_label.set_markup(&activeuse_format);
        //add to infobar empty area
	                  	 infobar123.content_area().add(&jobname_label); 
	                  	 infobar123.content_area().add(&jobsize_label);
	                  	 infobar123.content_area().add(&jobtype_label);
	                  	 infobar123.content_area().add(&activeuse_label);
	                  	  
	                  	 button.set_label("รับงาน");
	                  	 infobar123.add(&button);
	                  	// grid1.attach(&infobar123, 0, i.try_into().unwrap(), 1, 1);
	   //*id it's loop	to be create info on grid
       grid1.attach(&infobar123, 0, *id , 1, 1);
      
}	
	  
 grid1
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

}
