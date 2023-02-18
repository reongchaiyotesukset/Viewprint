use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
pub struct mySql{
	
}

#[derive(Debug, PartialEq, Eq)]
struct Example {
    id: i32,
    data: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct Checkid {
    id: i32,    
}

#[derive(Debug, PartialEq, Eq)]
struct Job {
    jobname: Option<String>,
    jobsize:  Option<String>,
    jobtype:	  Option<String>,
}

impl mySql{	
           	

fn connect_server() -> Result<PooledConn>
 {
	let url = "mysql://root:12345678@localhost:3306/supportprint";
	let pool = Pool::new(url)?;
	let mut conn = pool.get_conn()?;

		Ok(conn)
 }
   /*
	pub fn selectsql() -> std::result::Result<(), Box<dyn std::error::Error>>
	{
	let conn = Self::connect_server();
	let stack = conn?.query_map(
				"SELECT*from example",
				|(id, data)| {
					Example { id, data }
				},
			)?;
       
        println!("{:?}",stack[0].data);
	
	return Ok(());


	}
	*/
	pub fn select_return_String()-> String
	{
		   let conn = Self::connect_server();
         
          let stack : String  = conn.expect("Quey Error!")
				.exec_first::<String, &str, ()>("SELECT  job.jobname,job.jobsize,job.image,jobtype.typename,activeuse.activename 
												from job 
												left join jobtype on job.jobtype_id  = jobtype.jobtype_id 
												left join activeuse  on job.active_id  = activeuse.active_id", ())
				.unwrap()
				.unwrap()
				.to_string();
                 stack
             

	}
	
	pub fn select_return_Vec()
	{
		let conn = Self::connect_server();

     
	   let stack = conn.expect("REASON").query_map(
				"SELECT*from job",
				|(jobname, jobsize, jobtype)| {
					Job { jobname, jobsize,jobtype }
				},
			);
			
			       //stack     
         
	}
}
