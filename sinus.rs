// ---------------------------------------------------------
//
// Sinus
//
// ---------------------------------------------------------
#![allow(mixed_script_confusables)] // use π in code
use gtk::prelude::*;

const APP_ID: &str = "org.gtk_rs.sinus";

fn main() -> gtk::glib::ExitCode {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &gtk::Application) {
    //
    // définition CSS
    //
    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("sinus.css"));
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    //
    // charge UI
    //
    let builder = gtk::Builder::from_string(include_str!("sinus.ui"));
    //
    // window
    //
    let window = builder
        .object::<gtk::ApplicationWindow>("window")
        .expect("Couldn't get window");
    window.set_application(Some(app));
    // label
    let label_1 = builder
        .object::<gtk::Label>("label_1")
        .expect("Couldn't get label");
    // drawing_area
    let drawing_area = builder
        .object::<gtk::DrawingArea>("drawing_area")
        .expect("Couldn't get label");
    //
    // actions
    //
    drawing_area.set_draw_func(move |_area, c, w, h| {
        // dims ds le label
        label_1.set_text(format!("dim = {}x{}", w, h).as_str());
        // dessine
        c.set_source_rgb(1.0, 0.5, 0.5);
        c.rectangle(f64::from(w) - 20.0, f64::from(h) - 20.0, 10.0, 10.0);
        c.stroke().unwrap();
        c.set_source_rgb(0.1, 0.1, 0.1);
        c.move_to(10.0, f64::from(h) / 2.0);
        c.line_to(f64::from(w) - 10.0, f64::from(h) / 2.0);
        c.move_to(10.0, 10.0);
        c.line_to(10.0, f64::from(h) - 10.0);
        c.stroke().unwrap();
        // dessin suivant dimension
        if w > 400 {
            c.set_source_rgb(1.0, 0.5, 0.5);
            c.rectangle(f64::from(w) - 40.0, 10.0, 30.0, 30.0);
            c.stroke().unwrap();
        }
        // courbe Sinus
        c.set_source_rgb(1.0, 0.1, 0.1);
        let π: f64 = std::f64::consts::PI;
        let mut xx: f64 = 0.0;
        let max_xx: f64 = 1.0 * π;
        let pas_xx: f64 = 0.001;
        // definition fonction par une closure
        let f = |x: f64| -> f64 { f64::sin(x) * f64::sin(25.0 * x) }; // fonction sinus
                                                                      //let f = |x: f64| -> f64 {f64::cos(x)}; // fonction cosinus
                                                                      //xx = -π/2.0 + 0.2; max_xx = π/2.0 - 0.2;
                                                                      //let f = |x: f64| -> f64 {f64::tan(x)}; // fonction tangente
                                                                      // loop calcul point x,y
        c.move_to(
            10.0,
            f64::from(h) / 2.0 - f(xx) * (f64::from(h) - 20.0) / 2.0,
        );
        while xx <= max_xx {
            xx += pas_xx;
            let x = 10.0 + xx * (f64::from(w) - 20.0) / max_xx;
            let y = f64::from(h) / 2.0 - f(xx) * (f64::from(h) - 20.0) / 2.0;
            c.line_to(x, y);
        }
        c.stroke().unwrap();
    });
    // Present window
    window.present();
}
