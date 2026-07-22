use gtk4::prelude::*;
use gtk4::Application;
use gtk4::ApplicationWindow;
use gtk4::Button;
use gtk4::AspectFrame;
use std::rc::Rc;
use std::cell::RefCell;
use std::process::Command;
use std::time::Duration;
use std::io::Read;
use std::sync::mpsc;
use std::thread;
use gtk4::{CssProvider, gdk::Display, STYLE_PROVIDER_PRIORITY_APPLICATION};
mod config;
use config::AppConfig;



const BUTTON_ACTIONS : &[&str] = &["Volume +", "Volume -", "Volume + of specific Application", "Volume - of specific Application", "simulate Keypress", "Execute a CMD"];

struct SubmenuConfigSwitch {
    page_name: String,
    title:  String,
   
}

fn build_submenu_switch(config: &SubmenuConfigSwitch, stack: &gtk4::Stack, app_config: &Rc<RefCell<AppConfig>>,) -> gtk4::Box {
    let site = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
        //topbar gets created to structure the back button and possible title
        let topbar = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
        site.append(&topbar);
            // Back button and its functionality gets created
            let (buttonback) = gtk4::Button::new();
            buttonback.add_css_class("buttonback");
            buttonback.set_label("Back");
            topbar.append(&buttonback);
            let stack_clone = stack.clone();
            buttonback.connect_clicked(move |_button| {
            stack_clone.set_visible_child_name("home");
            });
            // Title of Site gets created 
            let title_label = gtk4::Label::new(Some(&config.title));
            title_label.add_css_class("text");
            topbar.append(&title_label);

        let dropdown = gtk4::DropDown::from_strings(BUTTON_ACTIONS);
        dropdown.add_css_class("dropdown");
site.append(&dropdown);

// restore last saved selection
let saved_index = app_config.borrow()
    .submenu_selections
    .get(&config.page_name)
    .copied()
    .unwrap_or(0);
dropdown.set_selected(saved_index as u32);

// save on every future change
let app_config_clone = Rc::clone(app_config);
let page_name_clone = config.page_name.clone();
dropdown.connect_selected_notify(move |dd| {
    let mut cfg = app_config_clone.borrow_mut();
    cfg.submenu_selections.insert(page_name_clone.clone(), dd.selected() as usize);
    cfg.save("config.json");
});



    stack.add_named(&site, Some(&config.page_name));

    site
}


fn main() {
    let app: Application = Application::builder()
        .application_id("macro.buddy") 
        .build();

    let (tx, rx) = mpsc::channel::<String>();

    // logic for serial port reading needs to be here




app.connect_activate(|app: &Application| {
    load_css();

    let cfg = AppConfig::load("config.json");
    let app_config = AppConfig::load("config.json");
    let app_config = Rc::new(RefCell::new(app_config));

    let window: ApplicationWindow = ApplicationWindow::new(app);
    window.set_default_size(800, 550);
    window.set_title(Some("MacroBuddy"));
    window.set_resizable(false);
    window.add_css_class("window");

    let header_bar = gtk4::HeaderBar::new();
    header_bar.add_css_class("titlebar");

    window.set_titlebar(Some(&header_bar));
    

    let buddy = gtk4::Box::new(gtk4::Orientation::Vertical, 40);

    let stack = gtk4::Stack::new();
    stack.add_named(&buddy, Some("home"));
    stack.set_visible_child_name("home"); 

    let top_bar = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    buddy.append(&top_bar);

    let main = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);
    buddy.append(&main);

    let profiles = ["Profile1", "Profile2", "Profile3", "Profile4"];
    let dropdown = gtk4::DropDown::from_strings(&profiles);
    dropdown.add_css_class("dropdown");
    top_bar.append(&dropdown);

    let current_profile_index = Rc::new(RefCell::new(0usize));

   
    let current_profile_index_clone = Rc::clone(&current_profile_index);
  

    let left_panel = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    main.append(&left_panel);
    left_panel.set_margin_top(40);


    left_panel.set_margin_start(100);

    //let title_label = gtk4::Label::new(Some("Select Profile"));
   // left_panel.append(&title_label);

    

    let right_panel = gtk4::Box::new(gtk4::Orientation::Vertical, 30);
    right_panel.set_halign(gtk4::Align::Center);
    right_panel.set_valign(gtk4::Align::Center);
    main.append(&right_panel);
    right_panel.set_margin_end(0);
    right_panel.set_margin_top(20);
    
    
    let row1 = gtk4::Box::new(gtk4::Orientation::Horizontal, 25);
    right_panel.append(&row1);
    row1.set_hexpand(true);
    row1.set_halign(gtk4::Align::Fill);

        let (button1, frame1) = make_square("1");
        row1.append(&frame1);
        let app_config_clone = Rc::clone(&app_config);
        let stack_clone = stack.clone();
        let current_profile_index_clone = Rc::clone(&current_profile_index);
        button1.connect_clicked(move |_button| {
            let cfg = app_config_clone.borrow();
            let index = *current_profile_index_clone.borrow();
            let current_profile = &cfg.profiles[index];
            let target = &current_profile.buttons[0];
            stack_clone.set_visible_child_name(target);

        });
        
        
        let (button2, frame2) = make_square("2");
        row1.append(&frame2);
        let app_config_clone = Rc::clone(&app_config);
        let current_profile_index_clone = Rc::clone(&current_profile_index);
        let stack_clone = stack.clone();
        button2.connect_clicked(move |_button| {
            let cfg = app_config_clone.borrow();
            let index = *current_profile_index_clone.borrow();
            let current_profile = &cfg.profiles[index];
            let target = &current_profile.buttons[1];
            stack_clone.set_visible_child_name(target);

        });

        let (button3, frame3) = make_square("3");
        row1.append(&frame3);
        let current_profile_index_clone = Rc::clone(&current_profile_index);
        let app_config_clone = Rc::clone(&app_config);
        let stack_clone = stack.clone();
        button3.connect_clicked(move |_button| {
            let cfg = app_config_clone.borrow();
            let index = *current_profile_index_clone.borrow();
            let current_profile = &cfg.profiles[index];
            let target = &current_profile.buttons[2];
            stack_clone.set_visible_child_name(target);

        });

    let row2 = gtk4::Box::new(gtk4::Orientation::Horizontal, 25);
    right_panel.append(&row2);
    row2.set_hexpand(true);
    row2.set_halign(gtk4::Align::Fill);

        let (button4, frame4) = make_square("4");
        row2.append(&frame4);
        let current_profile_index_clone = Rc::clone(&current_profile_index);
        let app_config_clone = Rc::clone(&app_config);
        let stack_clone = stack.clone();
        button4.connect_clicked(move |_button| {
            let cfg = app_config_clone.borrow();
            let index = *current_profile_index_clone.borrow();
            let current_profile = &cfg.profiles[index];
            let target = &current_profile.buttons[3];
            stack_clone.set_visible_child_name(target);

        });
        
        let (button5, frame5) = make_square("5");
        row2.append(&frame5);
        let current_profile_index_clone = Rc::clone(&current_profile_index);
        let app_config_clone = Rc::clone(&app_config);
        let stack_clone = stack.clone();
        button5.connect_clicked(move |_button| {
            let cfg = app_config_clone.borrow();
            let index = *current_profile_index_clone.borrow();
            let current_profile = &cfg.profiles[index];
            let target = &current_profile.buttons[4];
            stack_clone.set_visible_child_name(target);

        });

        let (button6, frame6) = make_square("6");
        row2.append(&frame6);
        let current_profile_index_clone = Rc::clone(&current_profile_index);
        let app_config_clone = Rc::clone(&app_config);
        let stack_clone = stack.clone();
        button6.connect_clicked(move |_button| {
            let cfg = app_config_clone.borrow();
            let index = *current_profile_index_clone.borrow();
            let current_profile = &cfg.profiles[index];
            let target = &current_profile.buttons[5];
            stack_clone.set_visible_child_name(target);

        });


    let row3 = gtk4::Box::new(gtk4::Orientation::Horizontal, 25);
    right_panel.append(&row3);

        let(button7, dial1) = make_dial("1");
        row3.append(&dial1);

        let(button8, dial2) = make_dial("2");
        row3.append(&dial2);

        let(button9, dial3) = make_dial("3");
        row3.append(&dial3);

    window.set_child(Some(&stack));
    window.present();

   let configs: Vec<SubmenuConfigSwitch> = (1..=4)
    .flat_map(|group| {
        (1..=6).map(move |i| SubmenuConfigSwitch {
            page_name: format!("site{group}.{i}"),
            title: format!("Profile {group} Button {i}"),
        })
    })
    .collect();


for config in &configs {
    build_submenu_switch(config, &stack, &app_config);
}
});



app.run();

}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    gtk4::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn make_square(label: &str) -> (Button, AspectFrame) {
    let button = Button::new();
    button.set_label(label);
    button.add_css_class("square-button");

    let frame = AspectFrame::new(0.5, 0.5, 1.0, false); 
        
    frame.set_child(Some(&button));

    (button, frame)
    
}

fn make_dial(label: &str) -> (Button, AspectFrame) {
    let button = Button::new();
    button.set_label(label);
    button.add_css_class("dial-button");

    let frame = AspectFrame::new(0.5, 0.5, 1.0, false); 

    frame.set_child(Some(&button));


    (button, frame)
}
