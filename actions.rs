// ----------------------------------------------------------------------------
//
// Démonstration d'une action avec un paramètre
//
// ----------------------------------------------------------------------------

#![windows_subsystem = "windows"] // no console sur WINDOWS

use gtk::{gio::ActionEntry, glib, prelude::*};

const MESSAGE: &str = "Exit après 5: ";

fn main() -> glib::ExitCode {
    std::env::set_var("GTK_CSD", "0"); // style win11 sur WINDOWS
    let app = gtk::Application::builder()
        .application_id("org.gtk.actions")
        .build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    //
    // WIDGETS
    //
    let label = gtk::Label::builder().label(format!("{MESSAGE}0")).build();
    //
    let buttom = gtk::Button::builder()
        .label("- 1")
        .action_name("win.count")
        .action_target(&(-1i16).to_variant())
        .build();
    //
    let button = gtk::Button::builder()
        .label("+ 1")
        .action_name("win.count")
        .action_target(&1i16.to_variant())
        .build();
    //
    let buttop = gtk::Button::builder()
        .label("+ 5")
        .action_name("win.count")
        .action_target(&5i16.to_variant())
        .build();
    //
    let quitte = gtk::Button::builder()
        .label("Quitter")
        .action_name("win.quitter")
        .build();
    //
    const MARGE: i32 = 10;
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(MARGE)
        .margin_bottom(MARGE)
        .margin_start(MARGE)
        .margin_end(MARGE)
        .spacing(MARGE)
        .halign(gtk::Align::Fill)
        .build();
    gtk_box.append(&buttom);
    gtk_box.append(&button);
    gtk_box.append(&buttop);
    gtk_box.append(&label);
    gtk_box.append(&quitte);
    //
    // WINDOW
    //
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .width_request(400)
        .child(&gtk_box)
        .build();
    //
    // ACTIONS
    //
    let count = ActionEntry::builder("count")
        .parameter_type(Some(&i16::static_variant_type())) // format parametre
        .state(0i16.to_variant()) // valeur init
        .activate(glib::clone!(
            #[weak] window, move |_, action, parameter| {
                let mut statexxx = action // Get state
                    .state()
                    .unwrap()
                    .get::<i16>()
                    .expect("bad type of variant.");
                let parameter = parameter // Get parameter
                    .expect("Could not get parameter.")
                    .get::<i16>()
                    .expect("bad type of variant.");
                statexxx += parameter; // Increase state by parameter
                action.set_state(&statexxx.to_variant()); // and store state
                                                          // Update label with new state
                label.set_label(&format!("{MESSAGE}{statexxx}"));
                if statexxx > 5_i16 {
                    window.close();
                }
            }
        ))
        .build();
    //
    let quitter = ActionEntry::builder("quitter")
        .activate(glib::clone!(
            #[weak] window, move |_, _, _| {
                window.close();
            }
        ))
        .build();
    //
    window.add_action_entries([count, quitter]);
    //
    // PRESENT
    //
    window.present();
}
