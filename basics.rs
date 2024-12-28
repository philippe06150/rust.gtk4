use gtk::{gio, glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(None::<String>, gio::ApplicationFlags::FLAGS_NONE);
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("First GTK Program"));
    window.set_default_size(600, 400);
    //
    let button1 = gtk::Button::with_label("Bouton 1");
    //
    let button2 = gtk::Button::with_label("Quitter");
    //
    let lb = gtk::Label::builder().vexpand(true).build();
    //
    let da = gtk::DrawingArea::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
    //
    const MARGES: i32 = 3;
    let mygrid = gtk::Grid::builder()
        .margin_start(MARGES)
        .margin_end(MARGES)
        .margin_top(MARGES)
        .margin_bottom(MARGES)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .row_spacing(MARGES)
        .column_spacing(MARGES)
        .build();
    mygrid.attach(&button1, 0, 0, 1, 1);
    mygrid.attach(&lb, 0, 1, 1, 1);
    mygrid.attach(&button2, 0, 2, 1, 1);
    mygrid.attach(&da, 1, 0, 1, 3);
    //
    // signal "clicked" sur `button1`
    //
    button1.connect_closure(
        "clicked",
        false,
        glib::closure_local!(move |button: gtk::Button| {
            button.set_label("Hello");
        }),
    );
    //
    button2.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| window.destroy()
    ));
    //
    // click gesture sur DrawingArea
    //
    let gesture = gtk::GestureClick::new();
    // boutons souris u32 = PRIMARY=1, MIDDLE=2, SECONDARY=3
    // gesture.set_button(gtk::gdk::ffi::GDK_BUTTON_PRIMARY as u32) ou bien
    // gesture.set_button(gtk::gdk::BUTTON_PRIMARY) ou bien
    gesture.set_button(1u32); // et si 0 = tout bouton !
    gesture.connect_pressed(|gesture, n, x, y| {
        gesture.set_state(gtk::EventSequenceState::Claimed);
        println!("Nombre click = {} - Position = {:.0}x{:.0}", n, x, y);
    });
    da.add_controller(gesture); // Assigne gesture à da
    //
    // fenêtre
    //
    window.set_child(Some(&mygrid));
    window.present();
}
