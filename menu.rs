use gtk::{gdk::Display, gio, glib, prelude::*, Align::Start, /*Align::Center, Align::End*/};

// do it local not to add 1 file for two lines !
const CSS: &str = "
	.fenetre {border: solid 2px lightgrey;}
	.quitter:hover {color: red; font-weight: bold;}
";

fn main() {
    // Create a new application
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.menubar")
        .build();
    application.connect_startup(on_startup);
    application.connect_activate(on_activate);
    application.run();
	//
	println!("Ca commence");
}
//
// définition du Menu
//
fn on_startup(app: &gtk::Application) {
	//
	// actions du Menu
	//
    let about = gio::ActionEntry::builder("about")
        .activate(|_, _, _| println!("About was pressed"))
        .build();
    let quit = gio::ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();
    app.add_action_entries([about, quit]);
	//
	// menu : architecture
	//
    let menubar = {
        let file_menu = {
            let about_menu_item = gio::MenuItem::new(
				Some("About"), 
				Some("app.about") // action about
			);
            let quit_menu_item = gio::MenuItem::new(
				Some("Quit"), 
				Some("app.quit") // action quit
			);
			//
            let file_menu = gio::Menu::new();
            file_menu.append_item(&about_menu_item);
            file_menu.append_item(&quit_menu_item);
            file_menu
        };
        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);
        menubar
    };
    app.set_menubar(Some(&menubar));
}
//
// mise en forme de la page
//
fn on_activate(app: &gtk::Application) {
	//
    // importe CSS, boite noire pour moi :-(
	//
    let provider = gtk::CssProvider::new();
	provider.load_from_string(CSS);
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
	//
	// window
	//
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Exemple de Menu")
        .default_width(800)
        .default_height(600)
        .show_menubar(true)
        .build();
	//
	// widgets
	//
    let b1 = creer_bouton("Cacher menu");
	let b2 = creer_bouton("Afficher menu");
	let b3 = gtk::ToggleButton::builder().label("Plein écran").build();
	let ll = gtk::Label::builder().vexpand(true).build(); // cree un espace
	let bq = creer_bouton("Quitter");
	bq.add_css_class("quitter");
	//
	let dw = gtk::DrawingArea::builder()
		.hexpand(true).vexpand(true)
		.valign(gtk::Align::Fill).halign(gtk::Align::Fill)
		.build();
	dw.add_css_class("fenetre");
	//
	let toto = "Hello \nDolly \nPhilippe \nPierre \nJoseph \nAlexandre";
	let tv = gtk::TextView::builder()
		.hexpand(true).left_margin(5).right_margin(5)
		.build();
	tv.buffer().set_text(&toto);
	let sw = gtk::ScrolledWindow::builder()
		.hexpand(true).child(&tv).has_frame(true)
		.min_content_height(50).max_content_height(50)
		.build();
	// bouton choix couleur
    let mycolor = gtk::ColorDialog::new();
    mycolor.set_with_alpha(true);
    mycolor.set_modal(false);
    mycolor.set_title("Couleur fond fenêtre");
    let c1 = gtk::ColorDialogButton::new(Some(mycolor));
	c1.set_halign(gtk::Align::Center);
    let couleur = gtk::gdk::RGBA::parse("rgba(255,100,100,1.0)");
    c1.set_rgba(&(couleur.unwrap()));
	//
	let sc = gtk::Scale::with_range(gtk::Orientation::Horizontal,0.0,255.0,1.0);
	sc.set_draw_value(true);
	sc.set_value_pos(gtk::PositionType::Bottom);
	sc.set_digits(0);
	//sc.set_width_request(150);
	sc.set_value(128.0);
	//
	let sp = gtk::SpinButton::with_range(0.0,255.0,1.0);
	sp.set_value(128.0);
	//
	//                +bh-------------+
	// Boxes          |+bv1--+bv2----+|
	//                || b1  | dw    ||
	//                || b2  |+sw---+||
	//                || c1  ||tv   |||
	//                || b3  ||     |||
	//                || ll  ||     |||
	//                || bq  |+-----+||
	//                |+-----+-------+|
	//                +---------------+
	//
	let bv1 = creer_box(gtk::Orientation::Vertical);
	bv1.append(&(gtk::Label::builder().label("ReCoucou").halign(Start).build()));
	bv1.append(&(gtk::Label::with_mnemonic("Coucou")));
	bv1.append(&(gtk::Label::new(Some("Encore Coucou"))));
	bv1.append(&b1);
	bv1.append(&b2);
	bv1.append(&c1);
	bv1.append(&b3);
	bv1.append(&sc);
	bv1.append(&sp);
	bv1.append(&ll);
	bv1.append(&bq);
	//
	let bv2 = creer_box(gtk::Orientation::Vertical);
	bv2.set_hexpand(true);
	bv2.set_vexpand(true);
	bv2.append(&dw);
	bv2.append(&sw);
	//
	let bh = creer_box(gtk::Orientation::Horizontal);
	bh.set_margin_top(5);
	bh.set_margin_bottom(5);
	bh.set_margin_start(5);
	bh.set_margin_end(5);
	bh.append(&bv1);
	bh.append(&bv2);
	//
	// connects
	//
    b1.connect_clicked(glib::clone!(
        #[weak] window, move |_| {
            window.set_show_menubar(false);
        }
    ));
	//
	b2.connect_clicked(glib::clone!(
        #[weak] window, move |_| {
            window.set_show_menubar(true);
        }
    ));
	//
	b3.connect_clicked(glib::clone!(
        #[weak] window, move |b3| {
			if b3.is_active() {
				window.fullscreen();
				window.set_show_menubar(false);
				b3.set_label("Fenêtre");
			} else {
				window.unfullscreen();
				window.set_show_menubar(true);
				b3.set_label("Plein écran");
			}
        }
    ));
	//
    bq.connect_clicked(glib::clone!(
        #[weak] window, move |_| {
            window.close();
        }
    ));
	//
	dw.set_draw_func(glib::clone!(
		#[weak] c1, move |_drawing_area, cx, w, h| {
			let couleur = ( c1.rgba().red(), c1.rgba().green(), c1.rgba().blue() ); // f32
			affichage(cx, w, h, couleur);
		}),
	);
	//
	c1.connect_rgba_notify(glib::clone!(#[weak] dw, move |_| dw.queue_draw() ));
	//
	sc.connect_value_changed(glib::clone!(#[weak] b1, #[weak] sp, move |sc| {
		//b1.set_label( &(format!("{}",sc.value())) );
		b1.set_label( &sc.value().to_string() );
		sp.set_value(sc.value());
	}));
	//
	sp.connect_value_changed(glib::clone!(#[weak] sc, move |sp| {
		sc.set_value(sp.value());
	}));
	//
	// window
	//
    window.set_child(Some(&bh));
    window.present();
}

fn creer_bouton(mes: &str) -> gtk::Button {
	gtk::Button::builder()
		.label(mes)
		.hexpand(false)
		.halign(gtk::Align::Fill) // Fill,Start,End,Center,
		.build()
}

fn creer_box(o: gtk::Orientation) -> gtk::Box {
	const MARGE: i32 = 5;
	gtk::Box::builder()
		.orientation(o)
		.spacing(MARGE)
		.build()
}

fn affichage (cx: &gtk::cairo::Context, w: i32, h: i32, couleur: (f32,f32,f32)) {
	let (r,v,b) = couleur;
	let l: f64 = 10.0; // largeur carrés
	cx.set_source_rgba(0.2, 0.2, 0.2, 1.0); // couleur background
	cx.paint().unwrap(); // peint le fond du drawarea
	cx.set_source_rgb(r.into(), v.into(), b.into()); // couleur pixel
	cx.set_line_width(1.0);
	for i in 1..10 {
		cx.rectangle(
			l*(i as f64), 
			l*(i as f64),
			(w as f64) - 2.0*l*(i as f64), 
			(h as f64) - 2.0*l*(i as f64)
		);
	}
	cx.stroke().unwrap();
}
