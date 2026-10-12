use gtk4::{prelude::*, glib, gio, gdk, cairo};
use rand;

fn main() -> glib::ExitCode {
    let app = gtk4::Application::new(
        None::<String>, 
        gio::ApplicationFlags::FLAGS_NONE
    );
    app.connect_startup(start_ui);
    app.connect_activate(build_ui);
    app.run()
}

fn start_ui(app: &gtk4::Application) {
	//
	// CSS
	//
	let provider = gtk4::CssProvider::new();
	provider.load_from_string("
		.toto { border: groove 3px lightblue; }
		.tata { border-radius: 5px; }
        /* La glissière (barre d'arrière-plan) */
        scale.custom-red-scale trough {
            background-color: #000000;
        }
        /* La partie remplie à gauche du curseur */
        scale.custom-red-scale highlight {
            background-color: #aaaaff;
        }
		/* étiquettes de texte attachées aux marques du scale */
		scale.custom-red-scale marks label {
			color: #0000ff;      /* texte */
			font-weight: bold;   /* (Optionnel) Met le texte en gras */
		}
		/* Cible les petits traits physiques de graduation */
		scale.custom-red-scale marks {
			color: #cc0000;
		}
		/* Cible les petits traits physiques de graduation */
		scale.custom-red-scale {
			color: #ffaaaa;
		}
	");
	gtk4::style_context_add_provider_for_display(
		&gdk::Display::default().expect("not connect to a display."),
		&provider,
		gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
	//
	// ACTIONS
	//
	let action_quit = gio::SimpleAction::new("quit", None);
	action_quit.connect_activate(glib::clone!(
		#[weak] app, move |_, _| app.quit()
	));
	app.add_action(&action_quit);
	//
	let action_label = gio::SimpleAction::new("label", None);
	action_label.connect_activate(glib::clone!(
		#[weak] app, move |_, _| {
			if let Some(window) = app.windows().first() {
				window
					.child().and_downcast::<gtk4::Grid>().unwrap()
					//.child_at(0, 0).unwrap()
					//.downcast_ref::<gtk4::Button>().unwrap()
					.child_at(0, 0).and_downcast::<gtk4::Button>().unwrap()
					.set_label("Label après menu");
			}
		}
	));
	app.add_action(&action_label);
	//
	// ActionEntry pour récupérer paramètre
	let app_clone = app.clone();
    let colorier = gio::ActionEntry::builder("colorier")
        .parameter_type(Some(&u32::static_variant_type()))
        .activate(move |_ , _action, parameter| {
            if let Some(level) = parameter.and_then(|p| p.get::<u32>()) {
				app_clone.windows().first().unwrap()
					.child().and_downcast::<gtk4::Grid>().unwrap()
					.child_at(0,4).and_downcast::<gtk4::DropDown>().unwrap()
					.set_selected(level);
            }
        })
        .build();
	app.add_action_entries([colorier]);
	//
	//
}

fn build_ui(app: &gtk4::Application) {
	//
	// MENU
	//
	let submenu = gio::Menu::new();
	// par défaut le paramètre numérique est i32
	// il faut donc typer façon C !!!
	submenu.append( Some("Rouge"), 	Some("app.colorier(uint32 0)"));
	submenu.append( Some("Vert"), 	Some("app.colorier(uint32 1)"));
	submenu.append( Some("Bleu"), 	Some("app.colorier(uint32 2)"));
	submenu.append( Some("Jaune"), 	Some("app.colorier(uint32 3)"));
	submenu.append( Some("Orange"), Some("app.colorier(uint32 4)"));
	//
	let menu = gio::Menu::new();
	menu.append( Some("Changer label du bouton"), Some("app.label") );
	let section1 = gio::Menu::new();
	for i in 1..5 {
		section1.append_submenu( Some(&format!("Choix DropDown {}", i)), &submenu );
		section1.append( Some(&format!("Menu {}", i)), None );
	}
	menu.append_section(Some("Exemple de section"), &section1);
	menu.append( Some("Quitter"), Some("app.quit") );
	//
    let menubutton = gtk4::MenuButton::builder()
		.icon_name("open-menu-symbolic")
		.css_classes(["circular",]) // bouton rond
		.has_frame(true)
		.visible(true)
		.direction(gtk4::ArrowType::Right)
		.menu_model(&menu)
		.build();
	//
	let header_bar = gtk4::HeaderBar::new();
    header_bar.pack_start(&menubutton);
    //
    // WIDGETS
    //
    let button1 = gtk4::Button::with_label("Label de départ");
    //
    let button2 = gtk4::Button::with_label("Cercles random");
    //
    let quitter = gtk4::Button::with_label("Quitter");
    //
    let lb = gtk4::Label::builder()
		.vexpand(true)
		.label("Position\nsouris")
		.build();
	//
	let cd = gtk4::ColorDialog::builder()
		.title("Choisir une couleur")
		.with_alpha(false)
		.build();
	let cdb = gtk4::ColorDialogButton::builder()
		.dialog(&cd)
		.rgba(&gdk::RGBA::new(0.6, 0.76, 0.95, 1.0))
		.build();
	//
	let cdb1 = gtk4::ColorDialogButton::builder()
		.dialog(&cd)
		.rgba(&gdk::RGBA::new(0.9, 0.1, 0.1, 1.0))
		.build();
    //
    let label_fond = gtk4::Label::new(Some("Fond du dessin: "));
    //
    let label_cercles = gtk4::Label::new(Some("Couleurs cercles: "));
    //
    let boxh1 = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    boxh1.set_halign(gtk4::Align::End);
    boxh1.set_hexpand(false);
    boxh1.append(&label_fond);
    boxh1.append(&cdb);
    //
    let boxh2 = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    boxh2.set_halign(gtk4::Align::End);
    boxh2.set_hexpand(false);
    boxh2.append(&label_cercles);
    boxh2.append(&cdb1);
    //
    let options = vec!["Rouge", "Vert", "Bleu", "Jaune","Orange"];
    let model = gtk4::StringList::new(&options);
    let drop_down = gtk4::DropDown::builder()
        .model(&model)            // Associe la liste de données
        .enable_search(true)      // Active barre de recherche interne
        .selected(0)              // select 1er élément
        .margin_top(0)
        .margin_bottom(0)
        .margin_start(0)
        .margin_end(0)
        .halign(gtk4::Align::Fill)
        .hexpand(false)
        .build();
    // déclaration du bloc scale avec bind pour lier
    let myadj = gtk4::Adjustment::new(50.0, 0.0, 100.0, 1.0, 10.0, 0.0);
	let myscale = gtk4::Scale::new(gtk4::Orientation::Horizontal, Some(&myadj));
	myscale.set_halign(gtk4::Align::Fill);
	myscale.set_hexpand(true);
	let myscalebutton = gtk4::ScaleButton::builder()
		.adjustment(&myadj)
		.icons(vec!("starred".to_string()))
		.build();
	let mylabelscale = gtk4::Label::new(Some(""));
    myscale.adjustment()
        .bind_property("value", &mylabelscale, "label")
        .flags(glib::BindingFlags::SYNC_CREATE)
        .transform_to(|_binding, value: &glib::Value| {
            let numeric_value = value.get::<f64>().unwrap_or(0.0);
            let formatted_text = format!("{:.0}", numeric_value);
            Some(formatted_text.to_value())
        })
        .build();
    // box incorporant les scale
    let boxh3 = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    boxh3.set_halign(gtk4::Align::Fill);
    boxh3.set_hexpand(false);
    boxh3.append(&myscalebutton);
    boxh3.append(&myscale);
    boxh3.append(&mylabelscale);
    //
    let secondscale = gtk4::Scale::with_range(
		gtk4::Orientation::Vertical,
		0.0,
		100.0,
		1.0
	);
	secondscale.set_size_request(-1, -1);
	secondscale.set_halign(gtk4::Align::Start);
	secondscale.add_css_class("custom-red-scale");
	secondscale.set_inverted(true);
    secondscale.set_draw_value(true);
    secondscale.set_value_pos(gtk4::PositionType::Right);
    secondscale.set_digits(0);
    secondscale.adjustment().set_value(50.0);
    let marks: Vec<f64> = (0..=100).step_by(10).into_iter().map(|x| x as f64).collect();
    for &val in &marks {
        let label = format!("{}", val);
        secondscale.add_mark(
            val,
            gtk4::PositionType::Left, // graduations à gauche
            Some(&label)
        );
    }
    secondscale.adjustment()
        .bind_property("value", &myscale.adjustment(), "value")
        .flags(glib::BindingFlags::BIDIRECTIONAL | glib::BindingFlags::SYNC_CREATE)
        .build();
    //
    let myswitch1 = gtk4::Switch::new();
    myswitch1.set_active(true);
    myswitch1.set_halign(gtk4::Align::Start);
    //
    let myswitch2 = gtk4::Switch::new();
    myswitch2.set_active(true);
    myswitch2.set_halign(gtk4::Align::Start);
    //
    let myswitch3 = gtk4::Switch::new();
    myswitch3.set_active(true);
    myswitch3.set_halign(gtk4::Align::Start);
    // liaison bidirectionnelle
    myswitch1
        .bind_property("active", &myswitch2, "active")
        .flags(glib::BindingFlags::BIDIRECTIONAL)
        .build();
    //
    myswitch1
        .bind_property("active", &myswitch3, "active")
        .flags(glib::BindingFlags::BIDIRECTIONAL)
        .build();
    // box incorporant les switch
    let boxh4 = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    boxh4.set_halign(gtk4::Align::Fill);
    boxh4.set_hexpand(false);
    boxh4.append(&myswitch1);
    boxh4.append(&myswitch2);
    boxh4.append(&myswitch3);
    //
    let da = gtk4::DrawingArea::builder()
        .vexpand(true)
        .hexpand(true)
        .css_classes(["toto", "tata"])
        .build();
    unsafe { 
		da.set_data(
			"message", 
			format!("GTK4 {}.{}.{} / version de l'application: {}", 
				gtk4::major_version(), 
				gtk4::minor_version(), 
				gtk4::micro_version(),
				env!("CARGO_PKG_VERSION")
				)
		);
		da.set_data("couleur", cdb.rgba());
		da.set_data("cercles", Vec::<f64>::new());
		da.set_data("c_cercles", cdb1.rgba());
	}
	//
	let text_view = gtk4::TextView::new();
	text_view.set_wrap_mode(gtk4::WrapMode::Word);
	text_view.buffer().set_text("Informations:\n");
	let scrolled_window = gtk4::ScrolledWindow::new();
	scrolled_window.set_policy(
		gtk4::PolicyType::Automatic, 
		gtk4::PolicyType::Automatic
	);
	scrolled_window.set_child(Some(&text_view));
	//
	let pane = gtk4::Paned::builder()
		.start_child(&da)
		.end_child(&scrolled_window)
		.resize_end_child(false)
		.wide_handle(true)
		.position(500)
		.build();
	//
	let seph = gtk4::Separator::new(gtk4::Orientation::Horizontal);
	//
	let mut mesinfos = "".to_string();
    match std::thread::available_parallelism() {
        Ok(num) => mesinfos = format!("Nombre de processeurs : {}", num),
        Err(e) => eprintln!("Impossible de détecter le nombre de processeurs : {}", e),
	}
	let infos = gtk4::Label::new(Some(&mesinfos));
	infos.set_halign(gtk4::Align::Start);
    //
    const MARGES: i32 = 6;
    let mygrid = gtk4::Grid::builder()
        .margin_start(MARGES)
        .margin_end(MARGES)
        .margin_top(MARGES)
        .margin_bottom(MARGES)
        .halign(gtk4::Align::Fill)
        .valign(gtk4::Align::Fill)
        .row_spacing(MARGES)
        .column_spacing(MARGES)
        .build();
    mygrid.attach(&button1,     0, 0, 1, 1);
    mygrid.attach(&button2,     0, 1, 1, 1);
    mygrid.attach(&boxh1,       0, 2, 1, 1);
    mygrid.attach(&boxh2,       0, 3, 1, 1);
    mygrid.attach(&drop_down,   0, 4, 1, 1);
    mygrid.attach(&boxh3,       0, 5, 1, 1);
    mygrid.attach(&boxh4,       0, 6, 1, 1);
    mygrid.attach(&lb,          0, 7, 1, 1);
    mygrid.attach(&quitter,     0, 8, 1, 1);
    mygrid.attach(&seph,        0, 9, 3, 1);
    mygrid.attach(&infos,       0,10, 3, 1);
    mygrid.attach(&secondscale, 1, 0, 1, 9);
    mygrid.attach(&pane,        2, 0, 1, 9);
    //
    const CERCLES: isize = 100;
    let titre = format!("GTK4 : {} cercles", CERCLES);
    let window = gtk4::ApplicationWindow::builder()
        .application(app)
        .title(&titre)
        .child(&mygrid)
        .titlebar(&header_bar)
        .default_width(900)
        .default_height(600)
        .build();
    //
    // signal "clicked" sur `button1`
    //
    drop_down.connect_selected_notify(glib::clone!(#[weak] infos, #[weak] text_view,  move |dropdown| {
        let index = dropdown.selected();
        let mut message = format!("Index sélectionné : {}", index);
		if let Some(item) = dropdown.selected_item() {
            let string_obj = item.downcast::<gtk4::StringObject>().unwrap();
            let texte = string_obj.string();
            text_view.buffer().insert(&mut text_view.buffer().end_iter(), &(texte.to_string() + "\n"));
            message += &format!(" ------- Texte récupéré : {}", texte);
		}
		infos.set_label(&message);
    }));
	//
    button1.connect_closure(
        "clicked", false,
        glib::closure_local!(move |button: gtk4::Button| {
            button.set_label("Label après click");
        }),
    );
    //
    button2.connect_clicked(glib::clone!(
		#[weak] da, move |_| {
			let mut toto = Vec::<f64>::new();
			let w = da.width() as f64;
			let h = da.height() as f64;
			for _ in 0..CERCLES {
				toto.push(rand::random_range(0f64..w));
				toto.push(rand::random_range(0f64..h));
			}
			unsafe {da.set_data("cercles", toto); }
			da.queue_draw();
		}
    ));
    //
    myswitch1.connect_notify_local(Some("active"), glib::clone!(#[weak] infos, move |s, _| {
		let m = format!("Etat du switch = {}", s.is_active());
		infos.set_label(&m);
	}));
    // ecrit position scale
    secondscale.connect_value_changed(glib::clone!(#[weak] text_view,#[weak] scrolled_window,  move |slider| {
        // La méthode .value() renvoie toujours un f64
        let position = slider.value();
        text_view.buffer().insert(
			&mut text_view.buffer().end_iter(), 
			&format!("{:.0}%\n", position)
		);
    }));
    // scroll auto en bas
    scrolled_window.vadjustment().connect_changed(|adj| {
        // Calcul du bas : upper - page_size
        let max_scroll = adj.upper() - adj.page_size();
        // applique le défilement que si effectif
        if max_scroll > 0.0 {
            adj.set_value(max_scroll);
        }
    });
    //
    quitter.connect_clicked(glib::clone!(
        #[weak] window, move |_| window.destroy()
    ));
    //
    // click gesture sur DrawingArea
    //
    let gesture = gtk4::GestureClick::new();
    // boutons souris u32 = PRIMARY=1, MIDDLE=2, SECONDARY=3
    // gesture.set_button(gtk4::gdk::ffi::GDK_BUTTON_PRIMARY as u32)
    // gesture.set_button(gtk4::gdk::BUTTON_PRIMARY) ou bien
    gesture.set_button(1u32); // et si 0 = tout bouton !
    //
    let lb_clone = lb.clone();
    let dac = da.clone();
    gesture.connect_pressed(move |gesture, n, x, y| {
        gesture.set_state(gtk4::EventSequenceState::Claimed);
        //
		let mes = format!("click = {}\nPos. = {:.0}x{:.0}", n, x, y);
		lb_clone.set_label(&mes);
        // simple click seulement
        if n == 1 {
			let mut vv: Vec<f64>;
			unsafe {vv = dac.data::<Vec<f64>>("cercles").unwrap().as_ref().to_vec();}
			vv.push(x);
			vv.push(y);
			unsafe { dac.set_data("cercles", vv); }
			dac.queue_draw();
		}
    });
    da.add_controller(gesture); // Assigne gesture à da
    //
    da.set_draw_func(dessin);
    //
    cdb.connect_rgba_notify(glib::clone!(#[weak] da, move |button| {
		unsafe { 
			da.set_data("couleur", button.rgba());
		}
		da.queue_draw();
    }));
    //
    cdb1.connect_rgba_notify(glib::clone!(#[weak] da, move |button| {
		unsafe { 
			da.set_data("c_cercles", button.rgba());
		}
		da.queue_draw();
    }));
    //
    // fenêtre
    //
    window.present();
}

fn dessin(
	da: &gtk4::DrawingArea,
	cr: &cairo::Context,
	w: i32,
	h: i32
) {
	let c: gdk::RGBA;
	unsafe {
		c = *da.data::<gdk::RGBA>("couleur").unwrap().as_ref();
	}
	cr.set_source_rgb(
		c.red() as f64, 
		c.green() as f64, 
		c.blue() as f64
	);
	cr.paint().unwrap();
	//
	cr.set_font_size(14.0);
	cr.set_source_rgb(0.1, 0.1, 0.1);
	//
	cr.move_to(10.0, 20.0);
	let message2: String;
	unsafe { 
		message2 = da.data::<String>("message")
			.unwrap()
			.as_ref()
			.to_string();
	}
	cr.show_text(&message2).unwrap();
	//
	cr.move_to(10.0, 40.0);
	let message1 = format!("{} x {}", w, h);
	cr.show_text(&message1).unwrap();
	//
	cr.move_to(10.0, 60.0);
	let message3 = format!("Fond = {:.2} / {:.2} / {:.2}", 
		c.red(), 
		c.green(), 
		c.blue()
	);
	cr.show_text(&message3).unwrap();
	//
	cr.stroke().unwrap();
	//
	let vv: Vec<f64>;
	unsafe {vv = da.data::<Vec<f64>>("cercles").unwrap().as_ref().to_vec();}
	let cc: gdk::RGBA;
	unsafe {
		cc = *da.data::<gdk::RGBA>("c_cercles").unwrap().as_ref();
	}
	cr.set_source_rgb(cc.red() as f64, cc.green() as f64, cc.blue() as f64);
	let taille: f64 = 3.0;
	for paire in vv.chunks_exact(2) {
		let x = paire[0]- taille; let y = paire[1] - taille;
		cr.rectangle(x, y, taille*2.0, taille*2.0);
		cr.stroke().unwrap();
		cr.arc(paire[0], paire[1], taille, 0.0, 6.29);
		cr.stroke().unwrap();
	}
}
