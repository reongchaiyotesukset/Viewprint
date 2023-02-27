use gtk::prelude::*;
use gtk::{InfoBar,Label,Button,Grid};
use mysql::*;
use mysql::prelude::*;

use crate::filesql::mySql;


#[derive(Debug, PartialEq, Eq)]
pub struct makeInfoBarDetial{
	
}
impl makeInfoBarDetial{	
                       	
                       	
 pub fn infobarDetial_config()-> Grid{ 
	 
		   let conn =  mySql::connect_server();
		   let grid1 = Grid::new();
	                                          
let sql_queryinfobar = Self::select_return_Vec();
			    
//for i  in 0..sql_queryinfobar.len()
//	{
  				
  					//let b : &str = &i.to_string();
  				 // println!("=============>>{:?}",sql_queryinfobar[i]);
  		
  	
for (id, jobname,jobsize,image,typename,activename) in &sql_queryinfobar 
{
					
		
  				  
 let infobar123 = InfoBar::new();

let jobname_label = Label::new(Some("NO LABEL1"));
let jobsize_label = Label::new(Some("NO LABEL2"));
let jobtype_label  =  Label::new(Some("NO LABEL3"));
let button = Button::new();

let jobname_format = &format!("{}{}{}","<span font_desc='Sans 25'>",jobname,"</span>");
let jobsize_format = &format!("{}{}{}","<span font_desc='Sans 25'>",jobsize,"</span>");
let typename_format = &format!("{}{}{}","<span font_desc='Sans 25'>",typename,"</span>");

		jobname_label.set_markup(&jobname_format);
		jobsize_label.set_markup(&jobsize_format);
		jobtype_label.set_markup(&typename_format);
 
		 infobar123.content_area().add(&jobname_label); 
		 infobar123.content_area().add(&jobsize_label);
		 infobar123.content_area().add(&jobtype_label);
		 button.set_label("OK");
		 infobar123.add(&button);
		// grid1.attach(&infobar123, 0, i.try_into().unwrap(), 1, 1);
       grid1.attach(&infobar123, 0, *id , 1, 1);
      
}		  
//}
//end loopl			
//return
 grid1
}
	
	
	pub fn select_return_Vec()-> Vec<(i32, String,String,String,String,String)>
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
         //data.push_str(&format!("{}{}{}{}{}{}{}\n"id,jobname,jobsize,image,typename,activename));
    }).unwrap();

	    result
} 

}
