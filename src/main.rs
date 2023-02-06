
use gtk::prelude::*;
use gtk::{ApplicationWindow, ShortcutsWindow};

use quick_xml::events::{ BytesText};
use quick_xml::writer::Writer;
use std::io::Cursor;

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
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &adw::Application) {


    let mut writer = Writer::new(Cursor::new(Vec::new()));
    writer.create_element("interface")
        .write_inner_content(|writer| {

            writer
                .create_element("object")
                .with_attribute(("class", "GtkShortcutsWindow"))
                .with_attribute(("id", "shortcuts"))

                .write_inner_content(|writer| {
                    writer
                        .create_element("property")
                        .with_attribute(("name", "modal"))
                        .write_text_content(BytesText::new("True"))?;

                    writer
                        .create_element("child")
                        .write_inner_content(|writer| {
                            writer
                                .create_element("object")
                                .with_attribute(("class", "GtkShortcutsSection"))
                                .write_inner_content(|writer| {
                                    writer
                                        .create_element("property")
                                        .with_attribute(("name", "section-name"))
                                        .write_text_content(BytesText::new("shortcuts"))?;

                                    writer
                                        .create_element("property")
                                        .with_attribute(("name","max-height"))
                                        .write_text_content(BytesText::new("10"))?;


                                    writer
                                        .create_element("child")
                                        .write_inner_content(|writer| {
                                            writer
                                                .create_element("object")
                                                .with_attribute(("class", "GtkShortcutsGroup"))
                                                .write_inner_content(|writer| {
                                                    writer
                                                        //NOTE maybe need more attributes
                                                        .create_element("property")
                                                        .with_attribute(("name", "title"))
                                                        .write_text_content(BytesText::new("General"))?;

                                                    writer
                                                        .create_element("child")
                                                        .write_inner_content(|writer| {
                                                            writer
                                                                .create_element("object")
                                                                .with_attribute(("class", "GtkShortcutsShortcut"))
                                                                .write_inner_content(|writer| {
                                                                    writer
                                                                        .create_element("property")
                                                                        .with_attribute(("name", "title"))
                                                                        .write_text_content(BytesText::new("Add new feed"))?;

                                                                    writer
                                                                        .create_element("property")
                                                                        .with_attribute(("name", "action-name"))
                                                                        .write_text_content(BytesText::new("blahbla"))?;

                                                                    writer
                                                                        .create_element("property")
                                                                        .with_attribute(("name", "accelerator"))
                                                                        .write_text_content(BytesText::new("&lt;primary&gt;a"))?;

                                                                    Ok(())
                                                                });

                                                            Ok(())
                                                        });

                                                    Ok(())

                                                });

                                            Ok(())
                                        });


                                    Ok(())
                                });


                            Ok(())
                        });

                    Ok(())
                });

            /*
            let fruits = ["apple", "orange"];
            for (quant, item) in fruits.iter().enumerate() {
                writer
                    .create_element("fruit")
                    .with_attribute(("quantity", quant.to_string().as_str()))
                    .write_text_content(BytesText::new(item))?;
            }
            */
            Ok(())
        });

    let result = writer.into_inner().into_inner();

    let s = match std::str::from_utf8(&result) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("JOJO {}",s);



    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    window.present();

    let shortcutswindow = gtk::builders::ShortcutsWindowBuilder::new().modal(true).build();

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
    let editors_group = gtk::builders::ShortcutsGroupBuilder::new().title("Editor").build();
    for (title, accelerator) in EDITOR_SHORTCUTS.iter() {
        editors_group.append(
            &gtk::builders::ShortcutsShortcutBuilder::new()
            .title(title)
            .accelerator(accelerator)
            .visible(true)
            .build(),
            );
    }
    let section = gtk::builders::ShortcutsSectionBuilder::new().max_height(10).section_name("section1").title("section1").build();
    shortcutswindow.set_child(Some(&section));

    section.append(&general_group);
    section.append(&editors_group);

    shortcutswindow.present();

    // Get `ShortcutsWindow` via `gtk::Builder`
    //let builder = gtk::Builder::from_string(include_str!("shortcuts.ui"));
    let builder = gtk::Builder::from_string(s);
    let shortcuts: ShortcutsWindow = builder
        .object("shortcuts")
        .expect("Could not get object `shortcuts` from builder.");

    //shortcuts.set_child(Some(&section));

    shortcuts.present();
}
