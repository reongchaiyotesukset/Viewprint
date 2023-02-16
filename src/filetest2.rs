use std::io::{self};
pub struct Foo(i32);
pub struct filetest2{}
impl filetest2{
	
   pub fn new()
   {
	   println!("TEST2");
   }
}

impl Foo{
	pub fn new() -> Self{
		Self(0)
	}
}
