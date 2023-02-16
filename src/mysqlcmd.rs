use gtk::prelude::*;
use gtk::{Grid};

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
	
	//test
	fn connect_server() -> Result<PooledConn>
{
let url = "mysql://root:12345678@localhost:3306/testdb";
let pool = Pool::new(url)?;
let mut conn = pool.get_conn()?;

     Ok(conn)
}
fn selectsql() -> std::result::Result<(), Box<dyn std::error::Error>>
{
let conn = connect_server();
let stack = conn?.query_map(
            "SELECT*from example",
            |(id, data)| {
                Example { id, data }
            },
        )?;
       
        println!("{:?}",stack[0].data);

return Ok(());

}
fn select_lastid() -> std::result::Result<(), Box<dyn std::error::Error>>
{
let conn = connect_server();

// let  stack: Vec<i32> = conn?.query("SELECT LAST_INSERT_ID();")?; value = 0;
 //let stack : Option<i32> = conn?.exec_first("SELECT LAST_INSERT_ID()", ()).unwrap();
 
//ok insert only conn?.exec_drop("INSERT INTO example (data) VALUES ('John Doe')",()).unwrap();

 
 let last_id : Option<i32>  = conn?
        .exec_first("SELECT LAST_INSERT_ID()", ())
        .unwrap()
        .unwrap();
       
     
    println!("The last inserted ID is: {:?}", last_id );
   
return Ok(());


}

fn insert_data2() -> std::result::Result<(), Box<dyn std::error::Error>>
{
let url = "mysql://root:12345678@localhost:3306/testdb";
let pool = Pool::new(url)?;
let mut conn = pool.get_conn()?;

conn.exec_drop("INSERT INTO example (data) VALUES ('testdb')",()).unwrap();
     //select_lastid();


let last_id : Option<i32>  = conn
        .exec_first("SELECT LAST_INSERT_ID()", ())
        .unwrap()
        .unwrap();
       
     
println!("The last inserted ID is: {:?}", last_id );
Ok(())  

}
fn selectinfomation() -> std::result::Result<(), Box<dyn std::error::Error>>
{
let url = "mysql://root:12345678@localhost:3306/testdb";
let pool = Pool::new(url)?;
let mut conn = pool.get_conn()?;



let data : Option<String>  = conn
        .exec_first("SELECT data from example ORDER BY data ASC", ())
        .unwrap()
        .unwrap();
           
println!("{:?}",data);
Ok(())  

}
fn insert_data(param_id :i32 ,param_data :String) -> std::result::Result<(), Box<dyn std::error::Error>> {


         
   /*      
     let insert_data = vec![
        Example{ id: param_id, data: Some(param_data.into())},
    ];
 
conn.exec_batch(
"INSERT INTO example(id, data)
VALUES (:id, :data)",
insert_data.iter().map(|p| params! {
"id" => p.id,
"data" => &p.data,
})
)?;
     */  
   
     Ok(())
   

}    
	//end			
		
}
