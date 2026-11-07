use gtk4::{gdk, glib, gio, cairo, prelude::*};
use std::cell::{Cell, RefCell};

thread_local!{
	static PROVIDER: RefCell<Option<gtk4::CssProvider>> = 
		const { RefCell::new(None) };
	static CHOIX: RefCell<String> = 
		const { RefCell::new(String::new()) };
	static CARRE: Cell<(bool,f64,f64,f64,f64)> =
		const { Cell::new( (false, 250.0, 50.0, 0.0, 0.0) ) };
}

fn main() -> glib::ExitCode {
    let application = gtk4::Application::builder()
        .application_id("com.gtk4-rs.css")
        .build();
    application.connect_startup(start_ui);
	application.connect_activate(build_ui);
    application.run()
}

fn start_ui(appli: &gtk4::Application) {
	//
	// CSS
	//
    let provider1 = gtk4::CssProvider::new();
	gtk4::style_context_add_provider_for_display(
		&gdk::Display::default().expect("no display."),
		&provider1,
		gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
	);
    provider1.load_from_string("
		entry.entry1 {
			background: linear-gradient(to right, #00f, #bbf);
			color: white;
			font-weight: bold;
			}
		button {
		}
		button.button1 {
			/* obligatoire pour changer le fond au survol */
			background-image: none;
			background-color: red;
		}
		button.button1:hover {
			transition: 500ms;
			background-color: lightgreen;
		}
		combobox button.combo box {
			padding: 5px;
		}
		combobox box arrow {
			-gtk-icon-source: -gtk-icontheme('pan-down-symbolic');
			border-left: 5px solid transparent;
			border-right: 5px solid transparent;
			border-top: 5px solid black;
		}
	");
	//
    PROVIDER.with(|cell| {
		let provider = gtk4::CssProvider::new();
		gtk4::style_context_add_provider_for_display(
			&gdk::Display::default().expect("no display."),
			&provider,
			gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
		);
        let mut borrow = cell.borrow_mut();
        *borrow = Some(provider);
    });
	css_modifiable("#6AA5EB");
	//
	// ACTIONS
	//
	let quit = gio::SimpleAction::new("quit", Default::default());
	quit.connect_activate(glib::clone!(#[weak] appli, move|_, _| 
		appli.quit()
	));
	appli.add_action(&quit);
	//
}

fn css_modifiable(moncss: &str) {
	PROVIDER.with(|cell| {
		let input = format!("#dessin {{ background-color: {}; }}", moncss);
		let provider2 = cell.borrow_mut();
		provider2.as_ref().unwrap().load_from_string( &input );
	});
}

fn build_ui(appli: &gtk4::Application) {
	//
    let entry = gtk4::Entry::new();
    entry.add_css_class("entry1");
    entry.set_text("Some text");
	//
    CHOIX.with(|cell| {
        let mut borrow = cell.borrow_mut();
        *borrow = "Bleu".to_string();
    });
    let model = gtk4::StringList::new(
		&["Bleu", "Rouge", "Vert"]
	);
    let drop_down = gtk4::DropDown::new(
		Some(model), 
		gtk4::Expression::NONE
	);
	//
	let espace = gtk4::Label::builder()
		.hexpand(true)
		.build();
	//
    let quitter = gtk4::Button::builder()
		.icon_name("application-exit")
		.css_classes(["button1", "circular"])
		.action_name("app.quit")
		.build();
    // box du haut
    let hbox1 = gtk4::Box::new(gtk4::Orientation::Horizontal, 5);
    hbox1.append(&entry);
    hbox1.append(&drop_down);
    hbox1.append(&espace);
    hbox1.append(&quitter);
    //
    let dessin = gtk4::DrawingArea::builder()
		.content_width(600)
		.content_height(400)
		.name("dessin")
		.hexpand(true)
		.vexpand(true)
		.build();
    // box du bas
    let hbox2 = gtk4::Box::builder()
		.spacing( 5 )
		.build();
	hbox2.append(&dessin);
    // box regroupant tout
    let vbox = gtk4::Box::builder()
		.orientation( gtk4::Orientation::Vertical )
		.spacing( 5 )
		.margin_top( 5 )
		.margin_bottom( 5 )
		.margin_start( 5 )
		.margin_end( 5 )
		.build();
	vbox.append(&hbox1);
	vbox.append(&hbox2);
    // fenêtre
    let window = gtk4::ApplicationWindow::builder()
		.application(appli)
		.title("CSS")
		.child(&vbox)
		.build();
	//
	let drag = gtk4::GestureDrag::new();
	drag.connect_drag_begin( glib::clone!(#[weak] dessin, move |_,x,y| {
		CARRE.with( |cell| {
			let (_ , x0, y0, _, _) = cell.get();
			if ((x-x0).abs() < 5.0) && ((y-y0).abs() < 5.0) {
				cell.set((true, x , y, 0.0, 0.0));
			}
		} );
		dessin.queue_draw();
	} ) );
	drag.connect_drag_update( glib::clone!(#[weak] dessin, move |_,dx,dy| {
		CARRE.with( |cell| {
			let (b , x, y, _, _) = cell.get();
			if b {
				cell.set((true, x , y, dx, dy));
			}
		} );
		dessin.queue_draw();
	} ) );
	drag.connect_drag_end( move |_,_,_| {
		CARRE.with( |cell| {
			let (b, x, y, dx, dy) = cell.get();
			if b {
				cell.set((false, x + dx, y + dy, 0.0, 0.0));
			}
		} );
	} );
	dessin.add_controller(drag);
	//
	dessin.set_draw_func(dessiner);
	//
	drop_down.connect_selected_notify( |a| {
		match a.selected() {
			0 => css_modifiable("#6AA5EB"),
			1 => css_modifiable("#F2463C"),
			2 => css_modifiable("lightgreen"),
			_ => {},
		}
		CHOIX.with(|cell| {
			let mut borrow = cell.borrow_mut();
			let binding = a.selected_item().unwrap()
				.downcast::<gtk4::StringObject>().unwrap().string();
			*borrow = (&binding).to_string();
		});
	});
	//
    window.present();
    //
	entry.set_position(-1); // annule le surlignage
}

fn dessiner (
	_: &gtk4::DrawingArea,
	cr: &cairo::Context,
	w: i32,
	h: i32
) {
	const GAUCHE: f64 = 5.0;
	let mes = format!("{} x {}", w, h);
	cr.set_font_size(16.0);
	cr.move_to(GAUCHE, 20.0);
	let _ = cr.show_text(&mes);
	//
	cr.move_to(GAUCHE, 40.0);
	CHOIX.with(|cell| {
		let mess = format!("Fond {}", cell.borrow_mut());
		let _ = cr.show_text(&mess);
	});
	// Définition des points pour la courbe de Bézier
	let start_x = 20.0;
	let start_y = h as f64 / 4.0;
	let mut ctrl1_x: f64 = 0.0;
	let mut ctrl1_y: f64 = 0.0; // Point de contrôle 1 (en haut)
	CARRE.with( |cell| {
		let (_,x,y,dx,dy) = cell.get();
		ctrl1_x = x + dx;
		ctrl1_y = y + dy;
	});
	let ctrl2_x = (w as f64 / 5.0) * 2.0;
	let ctrl2_y = h as f64 - 50.0; // Point de contrôle 2 (en bas)
	let end_x = w as f64 - 20.0;
	let end_y = h as f64 * 3.0 / 4.0;
	// Dessiner les lignes de contrôle
	cr.set_source_rgb(0.1, 0.1, 0.1);
	cr.set_line_width(1.0);
	cr.move_to(start_x, start_y);
	cr.line_to(ctrl1_x, ctrl1_y);
	cr.move_to(end_x, end_y);
	cr.line_to(ctrl2_x, ctrl2_y);
	cr.stroke().expect("Échec");
	// Dessine cercle saisie pt controle start
	cr.set_source_rgb(1.0, 1.0, 1.0);
	cr.arc(ctrl1_x, ctrl1_y, 5.0, 0.0, 6.29);
	cr.stroke().expect("Échec");
	cr.arc(ctrl2_x, ctrl2_y, 5.0, 0.0, 6.29);
	cr.stroke().expect("Échec");
	// Dessiner la courbe de Bézier principale en bleu
	cr.set_source_rgb(0.1, 0.4, 0.8); // Bleu
	cr.set_line_width(8.0);
	// On positionne le "pinceau" au point de départ
	cr.move_to(start_x, start_y);
	// curve_to prend : (contrôle_1_x, contrôle_1_y, contrôle_2_x, contrôle_2_y, fin_x, fin_y)
	cr.curve_to(ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, end_x, end_y);
	// Rendu du tracé
	cr.stroke().expect("Échec");
}
