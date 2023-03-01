extern crate gtk;
use gtk::prelude::*;
use gio::prelude::*;
use glib::prelude::*;
use gtk::{Application,Window, ApplicationWindow};
use crate::folder_infobardetial::makeInfoBarDetial;
use crate::filesql::mySql;

mod filewindows;
mod filesql;
mod folder_infobardetial;
mod fileheaderbar;
mod filegrid;
mod filefixed;
mod filelabel;
mod filestackswitcher;
mod filebutton;


fn main() {
	
    let app = Application::builder()
        .application_id("Application-testgtk")
        .build();
        
app.connect_activate(|app| {

let window = filewindows::Makewindow::window_config(app);
let header =  fileheaderbar::makeHeaderBar::headerBar_config();
let fixedmain =  filefixed::makefixed::fixed_config();
let label1 =  filelabel::makeLabel::label_config_1();
let label2 =  filelabel::makeLabel::label_config_2();
let label3 =  filelabel::makeLabel::label_config_3();
let label4 =  filelabel::makeLabel::label_config_4();     
let label5 =  filelabel::makeLabel::label_config_5();
let label6 =  filelabel::makeLabel::label_config_6();
let stackswitcher1 =  filestackswitcher::makeStackSwitcher::StackSwitcher_config();
let stack =  filestackswitcher::makeStackSwitcher::Stack_config();
let button =  filebutton::makeButton::button_config();

let gridmain =  filegrid::makeGrid::grid_config();

let infobarDetial = makeInfoBarDetial::infobarDetial_config();


fixedmain.add(&label1);
fixedmain.add(&label2);
fixedmain.add(&label3);
fixedmain.add(&label4);
fixedmain.add(&label5);
fixedmain.add(&label6);

stack.add_titled(&button, "page1", "งานที่เข้ามาใหม่วันนี้");
stack.add_titled(&infobarDetial, "page2", "งานที่รับไปแล้ววันนี้");
stackswitcher1.set_stack(Some(&stack));

gridmain.attach(&fixedmain, 0, 0 , 1, 1);
gridmain.attach(&stackswitcher1, 0, 1 , 1, 1);
gridmain.attach(&stack, 0, 2, 1, 1);




window.set_titlebar(Some(&header));
window.add(&gridmain);
window.show_all();
});
    app.run();

}
