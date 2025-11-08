use gettextrs::*;
use gtk::{gio::prelude::*, prelude::*, *};
use rfd::FileDialog;

const APP_ID: &str = "dev.luna.courseviewer";
static mut COURSE_FILE: String = String::new();

fn main() {
    println!("{}", gettext("Hello, world!"));
    
    // Create a new application
    let app: Application = Application::builder().application_id(APP_ID).build();
    
    // Connect to activate signal of app
    app.connect_activate(build_ui);
    
    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create button with label and margins
    let button: Button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    // Connect to clicked signal of button
    button.connect_clicked(|button: &Button| {
        // Set the label to hello world after pressing the button
        button.set_label(&gettext("Hello, World!"));
    });
    
    // Create a button to load a course
    let load_course: Button = Button::builder()
        .label(gettext("Open course"))
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    load_course.connect_clicked(|_button: &Button| {
        let file_path: Option<std::path::PathBuf> = FileDialog::new()
            .set_title(gettext("Choose a course file"))
            .pick_file();
        
        match file_path {
            Some(path) => {
                println!("File path: {:?}", path);
                unsafe {
                    COURSE_FILE = match path.into_os_string().into_string() {
                        Ok(path_str) => path_str,
                        Err(_e) => "".to_owned()
                    }
                }
            },
            None => println!("No file selected"),
        }
        
        #[allow(static_mut_refs)]
        unsafe { println!("{}", COURSE_FILE); }
    });
    
    // Create a navigation menu
    let navbar: Box = Box::new(Orientation::Vertical, 12);
    navbar.append(&button);
    
    // Create a course selection menu
    let csm: Grid = Grid::new();
    csm.attach(&load_course, 0, 0, 100, 100);
    
    // Create a box with navbar and content
    let main_box: Box = Box::new(Orientation::Horizontal, 12);
    main_box.append(&navbar);
    main_box.append(&csm);
    
    // Create a window and set the title
    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title(gettext("CourseViewer"))
        .child(&main_box)
        .maximized(true)
        .build();
    
    // Present window
    window.present();
}