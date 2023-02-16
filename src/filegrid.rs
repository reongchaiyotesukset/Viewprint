use gtk::prelude::*;
use gtk::{Grid};

pub struct makeGrid{
	
}
impl makeGrid{	
	
	   pub fn grid_config()-> Grid{ 
        let grid = Grid::builder()
            .margin(20)
            .margin_bottom(20)
            .margin_top(20)
            .margin_start(20)
            .margin_end(20)
            .build();
            grid
		}
}
