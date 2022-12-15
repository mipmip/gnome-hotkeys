
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, ShortcutsWindow};

const GENERAL_SHORTCUTS: [(&str, &str); 3] = [
    ("Open Command Pallette", "<Primary><Shift>P"),
    ("Quit", "<Primary>Q"),
    ("Re-execute Last Script", "<Primary><Shift>B"),
];

const EDITOR_SHORTCUTS: [(&str, &str); 12] = [
    ("Undo", "<Primary>Z"),
    ("Redo", "<Primary><Shift>Z"),
    ("Move line up", "<Alt>Up"),
    ("Move line down", "<Alt>Down"),
    ("Move cursor backwards one word", "<Primary>Left"),
    ("Move cursor forward one word", "<Primary>Right"),
    ("Move cursor to beginning of previous line", "<Primary>Up"),
    ("Move cursor to end of next line", "<Primary>Down"),
    ("Move cursor to beginning of line", "<Primary>Page_Up"),
    ("Move cursor to end of line", "<Primary>Page_Down"),
    ("Move cursor to beginning of document", "<Primary>Home"),
    ("Move cursor to end of document", "<Primary>End"),
];


const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() {
    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}


fn build_ui(app: &adw::Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();

    // Present window
    window.present();

    let shortcutswindow = ShortcutsWindow::builder()
        .build();

    let general_group = gtk::builders::ShortcutsGroupBuilder::new().title("General").build();
    for (title, accelerator) in GENERAL_SHORTCUTS.iter() {
        general_group.append(
            &gtk::builders::ShortcutsShortcutBuilder::new()
            .title(title)
            .accelerator(accelerator)
            .visible(true)
            .build(),
            );
    }
    let editors_group = gtk::builders::ShortcutsGroupBuilder::new().title("General").build();
    for (title, accelerator) in EDITOR_SHORTCUTS.iter() {
        editors_group.append(
            &gtk::builders::ShortcutsShortcutBuilder::new()
            .title(title)
            .accelerator(accelerator)
            .visible(true)
            .build(),
            );
    }
    let editors_group2 = gtk::builders::ShortcutsGroupBuilder::new().title("General").build();
    for (title, accelerator) in EDITOR_SHORTCUTS.iter() {
        editors_group2.append(
            &gtk::builders::ShortcutsShortcutBuilder::new()
            .title(title)
            .accelerator(accelerator)
            .visible(true)
            .build(),
            );
    }
    let editors_group3 = gtk::builders::ShortcutsGroupBuilder::new().title("General").build();
    for (title, accelerator) in EDITOR_SHORTCUTS.iter() {
        editors_group3.append(
            &gtk::builders::ShortcutsShortcutBuilder::new()
            .title(title)
            .accelerator(accelerator)
            .visible(true)
            .build(),
            );
    }

    let section = gtk::ShortcutsSection::builder()
        .build();

    section.append(&general_group);
    section.append(&editors_group);
    section.append(&editors_group2);
    section.append(&editors_group3);

//    section.show_all();

    shortcutswindow.set_child(Some(&section));

    shortcutswindow.present();



}
