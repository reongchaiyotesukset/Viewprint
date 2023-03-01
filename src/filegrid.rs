use gtk::prelude::*;
use gtk::{Grid};

pub struct makeGrid{
	
}
impl makeGrid{	
	
	   pub fn grid_config()-> Grid{ 
             let grid = Grid::builder()
             .build();   
            grid
		}
}
