use mysql::prelude::*;
use mysql::{from_row, Params};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

fn main() {
    // Create a GTK application
    let app = Application::new(Some("com.example.app"), Default::default()).unwrap();

    // Connect to the MySQL database
    let url = "mysql://username:password@localhost:3306/database";
    let pool = mysql::Pool::new(url).unwrap();

    // Execute a SQL query to fetch data from the database
    let mut conn = pool.get_conn().unwrap();
    let query = "";
    let rows = conn.exec_iter(query, Params::Empty).unwrap();

    // Create a GTK window and label to display the data
    let window = ApplicationWindow::new(&app);
    let label = Label::new(None);

    // Iterate over the rows and append the data to the label
    let mut data = String::new();
    for row in rows {
        let (col1, col2, col3) = from_row(row.unwrap());
        data.push_str(&format!("{} {} {}\n", col1, col2, col3));
    }
    label.set_text(&data);

    // Add the label to the window and display the window
    window.add(&label);
    window.show_all();

    // Run the GTK application
    app.run(&[]);
}
