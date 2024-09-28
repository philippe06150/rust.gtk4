#![allow(mixed_script_confusables)] // utilisation π

use gtk::{
    gdk::Display, glib, prelude::*, Align, Application, ApplicationWindow, Button,
    ColorDialogButton, CssProvider, DrawingArea, Frame, Grid, Label, Scale, Separator, TextView,
};
use rand;

const MA_MARGE: i32 = 4;
const MESSAGE: &str = "\
Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Sed non risus. Suspendisse lectus tortor, dignissim sit amet, \
adipiscing nec, ultricies sed, dolor. Cras elementum ultrices diam. \
Maecenas ligula massa, varius a, semper congue, euismod non, mi. \
Proin porttitor, orci nec nonummy molestie, enim est eleifend mi, \
non fermentum diam nisl sit amet erat. Duis semper. Duis arcu massa, \
scelerisque vitae, consequat in, pretium a, enim. Pellentesque congue. \
Ut in risus volutpat libero pharetra tempor. Cras vestibulum bibendum augue. \
Praesent egestas leo in pede. Praesent blandit odio eu enim. \
Pellentesque sed dui ut augue blandit sodales. \
Vestibulum ante ipsum primis in faucibus orci luctus et ultrices \
posuere cubilia Curae; Aliquam nibh. Mauris ac mauris sed pede pellentesque \
fermentum. Maecenas adipiscing ante non diam sodales hendrerit.\
";

fn main() -> glib::ExitCode {
    let appl = Application::builder()
        .application_id("org.example.gtk")
        .build();
    appl.connect_activate(build_ui);
    appl.run()
}

fn build_ui(app: &gtk::Application) {
    //
    // importe CSS, boite noire pour moi :-(
    //
    let provider = CssProvider::new();
    // Deprecated depuis: 4.12
    // provider.load_from_data(include_str!("style.css"));
    // maintenant
    provider.load_from_string(include_str!("courbe.css"));
    //
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    //
    // définition fenêtre
    //
    let window = ApplicationWindow::builder()
        .application(app)
        //.default_width(1840)
        //.default_height(400)
        //.title("gtk App")
        .build();
    window.set_title(Some("Editeur de courbes"));
    window.set_default_size(1200, 1);
    // https://gtk-rs.org/gtk-rs/git/docs/gtk/builders/struct.WindowBuilder.html#method.deletable
    window.set_deletable(true); // affichage ou pas bouton fenetre fermer
    window.set_resizable(true);
    window.set_opacity(1.0);
    window.set_decorated(true);
    window.add_css_class("fenetre");
    //window.add_action(&action_button_frame);
    //
    // WIDGETS
    //
    // construction grid
    let grid = Grid::builder()
        .margin_start(MA_MARGE)
        .margin_end(MA_MARGE)
        .margin_top(MA_MARGE)
        .margin_bottom(MA_MARGE)
        .row_spacing(MA_MARGE)
        .column_spacing(MA_MARGE)
        .build();
    //
    let vide = Label::new(Some(""));
    vide.set_vexpand(true);
    vide.set_valign(gtk::Align::Baseline);
    //
    let b1 = Button::with_label("Taille mini");
    b1.add_css_class("bouton");
    b1.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| window.set_default_size(1, 1)
    ));
    //
    let b2 = Button::with_label("Maximize");
    b2.add_css_class("bouton");
    //
    let b3 = Button::with_label("Texte");
    //
    let b4 = Button::with_label("Agrandir");
    //
    let b5 = Button::with_label("320x200");
    //
    let b6 = Button::with_label("1200x400");
    //
    let version = format!("Gtk4 {}.{}", gtk::major_version(), gtk::minor_version());
    let b7 = Button::with_label(&version);
    //
    let b8 = Button::with_label("");
    //
    let b9 = Button::with_label("");
    //
    let b10 = Button::with_label("AlertDial");
    //
    let mycolor = gtk::ColorDialog::new();
    mycolor.set_with_alpha(true);
    mycolor.set_modal(false);
    mycolor.set_title("Couleur fond fenêtre");
    let c1 = ColorDialogButton::new(Some(mycolor));
    // https://gtk-rs.org/gtk-rs/git/docs/gdk4/struct.RGBA.html
    // BLACK BLUE GREEN RED WHITE TRANSPARENT
    //let couleur = gtk::gdk::RGBA::parse("rgb(255,0,0)");
    //let couleur = gtk::gdk::RGBA::parse("#0000FF");
    let couleur = gtk::gdk::RGBA::parse("rgba(0,0,0,1.0)");
    c1.set_rgba(&(couleur.unwrap()));
    //
    let mut cc: Vec<ColorDialogButton> = Vec::new();
    for i in 0..=3 {
        let mycolor = gtk::ColorDialog::new();
        mycolor.set_with_alpha(false);
        mycolor.set_modal(true);
        mycolor.set_title("Paramètre");
        cc.push(ColorDialogButton::new(Some(mycolor)));
        let ma_couleur = format!(
            "rgb({},{},{})",
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>()
        );
        let couleur = gtk::gdk::RGBA::parse(&ma_couleur);
        cc[i].set_rgba(&(couleur.unwrap()));
    }
    // bouton EXIT
    let quit_button = Button::with_label("Q u i t");
    quit_button.add_css_class("monquit");
    //
    let sep = Separator::new(gtk::Orientation::Horizontal);
    //
    let sc1 = Scale::with_range(gtk::Orientation::Horizontal, 50.0, 150.0, 1.0);
    sc1.add_mark(50.0, gtk::PositionType::Bottom, Some("50"));
    sc1.add_mark(75.0, gtk::PositionType::Bottom, Some("75"));
    sc1.add_mark(100.0, gtk::PositionType::Bottom, Some("100"));
    sc1.add_mark(125.0, gtk::PositionType::Bottom, Some("125"));
    sc1.add_mark(150.0, gtk::PositionType::Bottom, Some("150"));
    sc1.set_draw_value(true);
    sc1.set_digits(0); // affichage x décimales
    sc1.set_value(100.0);
    sc1.set_value_pos(gtk::PositionType::Top);
    //
    let sc2 = Scale::with_range(gtk::Orientation::Horizontal, 50.0, 150.0, 10.0);
    sc2.set_draw_value(true);
    sc2.set_digits(0);
    sc2.set_value_pos(gtk::PositionType::Right);
    sc2.add_mark(50.0, gtk::PositionType::Top, Some("50"));
    sc2.add_mark(60.0, gtk::PositionType::Bottom, Some("60"));
    sc2.add_mark(70.0, gtk::PositionType::Top, Some("70"));
    sc2.add_mark(80.0, gtk::PositionType::Bottom, Some("80"));
    sc2.add_mark(90.0, gtk::PositionType::Top, Some("90"));
    sc2.add_mark(100.0, gtk::PositionType::Bottom, Some("100"));
    sc2.add_mark(110.0, gtk::PositionType::Top, Some("110"));
    sc2.add_mark(120.0, gtk::PositionType::Bottom, Some("120"));
    sc2.add_mark(130.0, gtk::PositionType::Top, Some("130"));
    sc2.add_mark(140.0, gtk::PositionType::Bottom, Some("140"));
    sc2.add_mark(150.0, gtk::PositionType::Top, Some("150"));
    sc2.set_draw_value(true);
    sc2.set_value(100.0);
    //
    let t1 = TextView::builder()
        .halign(Align::Start)
        .editable(false)
        .pixels_above_lines(0)
        .pixels_below_lines(1)
        .margin_start(5)
        .margin_end(5)
        .margin_bottom(5)
        .right_margin(190)
        .build();
    //let t1 = TextView::new();
    //t1.set_left_margin(0);
    //t1.set_right_margin(200);
    t1.set_editable(true);
    t1.add_css_class("mon_texte");
    t1.buffer().set_text("");
    //
    let fr = Frame::new(Some("Informations :"));
    fr.set_child(Some(&t1));
    //
    let area: DrawingArea = DrawingArea::new();
    area.add_css_class("dessin");
    area.set_content_width(600);
    area.set_content_height(400);
    area.set_hexpand(true);
    //area.set_vexpand(true); // ?? pas besoin ??
    // https://gtk-rs.org/gtk-rs/git/docs/gtk/enum.Align.html
    // Fill Start End Center Baseline
    area.set_valign(gtk::Align::Baseline);
    area.set_halign(gtk::Align::Baseline);
    //
    // CONNECTs
    //
    c1.connect_rgba_notify(glib::clone!(
        #[weak]
        area,
        move |_| area.queue_draw()
    ));
    //
    b2.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| {
            //window.maximize();
            //window.set_title(Some("...................."));
            match window.is_fullscreen() {
                false => window.fullscreen(),
                true => window.unfullscreen(),
            }
        }
    ));
    //
    b5.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| window.set_default_size(320, 200)
    ));
    //
    b3.connect_clicked(glib::clone!(
        #[weak]
        t1,
        move |_| t1.buffer().set_text("Philippe\nPierre\nJoseph\nAlexandre")
    ));
    //
    b6.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| window.set_default_size(1200, 400)
    ));
    //
    quit_button.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| window.destroy()
    ));
    //
    area.connect_resize(glib::clone!(
        #[weak]
        t1,
        move |_da, x, y| {
            let s = format!("{}x{}", x, y);
            t1.buffer().set_text(&s);
        }
    ));
    //
    sc1.connect_value_changed(glib::clone!(
        #[weak]
        t1,
        #[weak]
        area,
        move |myscale| {
            let s = format!("valeur SCALE 1 = {}", myscale.value());
            t1.buffer().set_text(&s);
            area.queue_draw();
        }
    ));
    //
    sc2.connect_value_changed(glib::clone!(
        #[weak]
        t1,
        #[weak]
        area,
        move |myscale| {
            let s = format!("valeur SCALE 2 = {}", myscale.value());
            t1.buffer().set_text(&s);
            area.queue_draw();
        }
    ));
    // change la taille du dessin par appui bouton 4
    b4.connect_clicked(glib::clone!(
        #[weak]
        area,
        move |_| area.set_content_width(200)
    ));
    //
    b10.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| {
            gtk::AlertDialog::builder()
                .modal(true)
                .message("Affichage de courbes")
                .detail(MESSAGE)
                .buttons(["Compris"]) // si omis, rajoute un bouton "Close", "fermer" en français
                .build()
                .show(Some(&window));
        }
    ));
    //
    area.set_draw_func(glib::clone!(
        #[weak]
        sc1,
        #[weak]
        sc2,
        #[weak]
        c1,
        move |_area, c, w, h| {
            let couleur = (c1.rgba().red(), c1.rgba().green(), c1.rgba().blue()); // f32
            affiche_courbe(c, w, h, sc1.value(), sc2.value(), couleur);
        }
    ));
    //
    // layout de la fenetre : grid - colonne - ligne
    // +-------+-------+-------+-----------+
    // |   b1  |   b2  |  c1   |           |
    // +-------+-------+-------+           |
    // |   b3  |   b4  | cc[0] |           |
    // +-------+-------+-------+           |
    // |   b5  |   b6  | cc[1] |           |
    // +-------+-------+-------+           |
    // |   b7  |   b8  | cc[2] |           |
    // +-------+-------+-------+           |
    // |   b9  |   b10 | cc[3] |    area   |
    // +-------+-------+-------+           |
    // |           sep         |           |
    // +-------+-------+-------+           |
    // |           sc1         |           |
    // +-----------------------+           |
    // |           sc2         |           |
    // +-----------------------+           |
    // |           fr(t1)      |           |
    // +-----------------------+           |
    // |          vide         |           |
    // +-----------------------+           |
    // |     quit_button       |           |
    // +-----------------------+-----------+
    //
    grid.attach(&b1, 0, 0, 1, 1);
    grid.attach(&b2, 1, 0, 1, 1);
    grid.attach(&b3, 0, 1, 1, 1);
    grid.attach(&b4, 1, 1, 1, 1);
    grid.attach(&b5, 0, 2, 1, 1);
    grid.attach(&b6, 1, 2, 1, 1);
    grid.attach(&b7, 0, 3, 1, 1);
    grid.attach(&b8, 1, 3, 1, 1);
    grid.attach(&b9, 0, 4, 1, 1);
    grid.attach(&b10, 1, 4, 1, 1);
    grid.attach(&c1, 2, 0, 1, 1);
    for i in 0..cc.len() {
        grid.attach(&cc[i], 2, 1 + (i as i32), 1, 1);
    }
    grid.attach(&sep, 0, 5, 3, 1);
    grid.attach(&sc1, 0, 6, 3, 1);
    grid.attach(&sc2, 0, 7, 3, 1);
    grid.attach(&fr, 0, 8, 3, 1);
    grid.attach(&vide, 0, 9, 2, 1); // vexpand
    grid.attach(&quit_button, 0, 10, 3, 1);
    grid.attach(&area, 3, 0, 1, 11); //expand
                                     // ajoute la grille à la fenetre
    window.set_child(Some(&grid));
    //
    // affiche la fenetre
    //
    window.present();
}

fn affiche_courbe(
    c: &gtk::cairo::Context,
    w: i32,
    h: i32,
    r1: f64,
    r2: f64,
    couleur: (f32, f32, f32),
) {
    let ww = w as f64;
    let hh = h as f64;
    let π: f64 = std::f64::consts::PI;
    let (r, v, b) = couleur;
    //
    c.set_source_rgb(0.0, 0.0, 1.0); //axes en bleu
                                     //
    c.line_to(10.0, hh / 2.0);
    c.line_to(ww - 10.0, hh / 2.0);
    c.stroke().unwrap();
    //
    c.line_to(ww - 20.0, hh / 2.0 - 5.0);
    c.line_to(ww - 10.0, hh / 2.0);
    c.line_to(ww - 20.0, hh / 2.0 + 5.0);
    c.stroke().unwrap();
    //
    c.line_to(10.0, 10.0);
    c.line_to(10.0, hh - 10.0);
    c.stroke().unwrap();
    //
    c.line_to(5.0, 20.0);
    c.line_to(10.0, 10.0);
    c.line_to(15.0, 20.0);
    c.stroke().unwrap();
    //
    c.set_source_rgb(r.into(), v.into(), b.into());
    //
    for x in 10..=(w - 10) {
        let xx = 2.0 * π * ((x as f64) - 10.0) / (ww - 20.0);
        let yy = f64::sin(xx * 100.0 / r1) * f64::sin(20.0 * xx * 100.0 / r2);
        c.line_to(x as f64, hh - 10.0 + ((20.0 - hh) * (yy + 1.0) / 2.0));
    }
    c.stroke().unwrap();
}
