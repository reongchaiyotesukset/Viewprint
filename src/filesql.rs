use mysql::*;
use mysql::prelude::*;

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
	fn selectsql() -> std::result::Result<(), Box<dyn std::error::Error>>
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
}
