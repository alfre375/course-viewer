use std::fs;

use gettextrs::*;
use gtk::{gio::prelude::*, prelude::*, *};
use rfd::FileDialog;
use serde_json::Value;

const APP_ID: &str = "dev.luna.courseviewer";
const ETC_PATH: &str = "/etc/course-viewer";
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
    
    // Create a navigation menu
    let navbar: Box = Box::new(Orientation::Vertical, 6);
    navbar.append(&button);
    
    // Create a course selection menu
    let csm: Box = Box::new(Orientation::Vertical, 6);
    csm.append(&load_course);
    
    // Create a box with navbar and content
    let main_box: Box = Box::new(Orientation::Horizontal, 6);
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
    
    load_course.connect_clicked(|_button: &Button| {
        let file_path: Option<std::path::PathBuf> = FileDialog::new()
            .set_title(gettext("Choose a course folder"))
            .pick_folder();
        
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
        let file_contents_res: Result<String, std::io::Error> = unsafe {
            fs::read_to_string(COURSE_FILE.clone() + "/metadata.json")
        };
        
        let mut parsed: Value = serde_json::from_str("{}").expect("Something went wrong");
        
        match file_contents_res {
            Ok(data) => parsed = serde_json::from_str(&data).expect("Something went wrong while parsing data"),
            Err(e) => println!("{:?}", e),
        }
        
        let lesson_selector: Box = Box::new(Orientation::Vertical, 6);
        let return_button: Button = Button::builder().label(gettext("Back to main menu")).build();
        lesson_selector.append(&return_button);
        
        println!("{:?}", parsed["course_layout"].as_array().expect("Didn't work"));
        for lesson in parsed["course_layout"].as_array().expect("Didn't work") {
            println!("Lesson: {}",lesson["name"].as_str().expect("Something went wrong"));
            let lesson_button: Button = Button::builder()
                .label(lesson["name"].as_str().expect("Something went wrong"))
                .build();
            lesson_selector.append(&lesson_button);
        }
        
        let window: ApplicationWindow = ApplicationWindow::builder()
            .maximized(true)
            .child(&lesson_selector)
            .title(parsed["course_name"].as_str().expect("Didn't work"))
            .build();
        
        window.present();
        
        println!("{:?}", parsed);
        
        #[allow(static_mut_refs)]
        unsafe { println!("{}", COURSE_FILE); }
    });
}