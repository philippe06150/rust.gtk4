use gtk4::{prelude::*, glib, gdk};

fn main() {
    let app = gtk4::Application::builder()
        .application_id("org.example.dropdown")
        .build();
    app.connect_startup(start_ui);
    app.connect_activate(build_ui);
    app.run();
}

fn start_ui(_: &gtk4::Application) {
	let provider = gtk4::CssProvider::new();
    gtk4::style_context_add_provider_for_display(
		&gdk::Display::default().expect("Could not connect display."),
		&provider,
		gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
	provider.load_from_string("
		#dessin {border-radius: 15px;}
	");
}

fn build_ui(app: &gtk4::Application) {
	//
    let options = ["Cercle", "Carré", "Losange", "Coeur",
		"Quitter"];
    let model = gtk4::StringList::new(&options);
    let drop_down = gtk4::DropDown::builder()
        .model(&model)            // Associe la liste de données
        .enable_search(false)      // Desactive barre de recherche
        .selected(0)              // select 1er élément
        .vexpand(false)
        .valign(gtk4::Align::Start)
        .build();
    //
    let info = gtk4::Entry::builder()
		.can_focus(false)
		.hexpand(true)
		.width_chars(40)
		.vexpand(false)
        .valign(gtk4::Align::Start)
		.build();
	//
	const MARGES_BOX: i32 = 6;
	let hbox = gtk4::Box::new(
		gtk4::Orientation::Horizontal, 
		MARGES_BOX
	);
	hbox.append(&drop_down);
	hbox.append(&info);
	//
	let dessin = gtk4::DrawingArea::builder()
		.hexpand(true)
		.vexpand(true)
		.content_height(300)
		.name("dessin")
		.build();
	//
	let vbox = gtk4::Box::builder()
		.orientation(gtk4::Orientation::Vertical)
		.spacing(MARGES_BOX)
        .margin_top(MARGES_BOX)
        .margin_bottom(MARGES_BOX)
        .margin_start(MARGES_BOX)
        .margin_end(MARGES_BOX)
		.build();
	vbox.append(&hbox);
	vbox.append(&dessin);
    // Configuration de la fenêtre principale
    let window = gtk4::ApplicationWindow::builder()
        .application(app)
        .title("Exemple GTK4 DropDown Builder")
        .default_width(-1)
        .default_height(-1)
        .child(&vbox)
        .build();
    // Écouter les changements de sélection
    drop_down.connect_selected_notify(glib::clone!(
	#[weak] app, 
	#[weak] info, 
	#[weak] dessin, 
	move |dropdown| {
        let index = dropdown.selected();
        if index==4 { app.quit() }
        info.buffer().set_text(
			&format!("Index sélectionné : {}", index)
		);
		if let Some(item) = dropdown.selected_item() {
            let string_obj = item.downcast::<gtk4::StringObject>()
				.unwrap();
            let texte = string_obj.string();
            info.insert_text(
				&format!(" / Texte récupéré : {}", texte),
				&mut ( info.text_length() as i32 )
			);
		}
		dessin.queue_draw();
    }));
    //
    dessin.set_draw_func(glib::clone!(
    #[weak] drop_down, 
    move |_, cr, w, h| {
		use std::f64::consts::{PI, FRAC_PI_2};
		// on clip pour garder bords arrondis du css
		let w = w as f64; 
		let h = h as f64;
		let radius = 15.0; // border-radius du CSS
		cr.new_sub_path();
		cr.arc(w - radius, radius, radius, -FRAC_PI_2, 0.0); // Haut-Droite
		cr.arc(w - radius, h - radius, radius, 0.0, FRAC_PI_2); // Bas-Droite
		cr.arc(radius, h - radius, radius, FRAC_PI_2, PI); // Bas-Gauche
		cr.arc(radius, radius, radius, PI, 3.0 * FRAC_PI_2); // Haut-Gauche
		cr.close_path();
		cr.clip();
		// fond en gris
		cr.set_source_rgb(0.3, 0.3, 0.3); // fond gris
		if let Err(_) = cr.paint() {} // fait rien si erreur
		//
		if let Some(item) = drop_down.selected_item() {
			if let Ok(obj) = item.downcast::<gtk4::StringObject>() {
				let toto = obj.string().to_string();
				//
				cr.set_font_size(16.0);
				cr.set_source_rgb(1.0, 1.0, 1.0);
				cr.move_to(20.0, 20.0);
				cr.show_text(&toto).unwrap();
				cr.stroke().unwrap();
				match toto.as_str() {
					"Cercle" => {
						// cercle centré
						let center_x = w / 2.0;
						let center_y = h / 2.0;
						let radius = f64::min(center_x, center_y) - 30.0; // Marge
						cr.set_source_rgb(171.0/255.0, 196.0/255.0, 32.0/255.0);
						cr.set_line_width(3.0);
						cr.arc(center_x, center_y, radius, 0.0, 2.0 * PI);
						if let Err(_) = cr.stroke() {} // fait rien si erreur
					},
					"Carré" => {
						// carré centré
						cr.set_source_rgb(171.0/255.0, 196.0/255.0, 32.0/255.0);
						cr.set_line_width(3.0);
						cr.rectangle(30.0, 30.0, w-60.0, h-60.0 );
						if let Err(_) = cr.stroke() {} // fait rien si erreur
					},
					_ => {},
				}
			}
		}
	}));
    //
    window.present();
}
