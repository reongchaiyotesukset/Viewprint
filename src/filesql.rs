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
impl mySql{	
           	

	fn connect_server() -> Result<PooledConn>
{
let url = "mysql://root:12345678@localhost:3306/testdb";
let pool = Pool::new(url)?;
let mut conn = pool.get_conn()?;

     Ok(conn)
}
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
	
	pub fn selectreturn()->i32
	{
	let a= 100;
	let conn = Self::connect_server();
	
       
        a

	}
}
