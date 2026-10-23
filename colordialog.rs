//
// marche avec rust 1.99.1
// & gtk4-rs 0.11.5 avec feature à v4_18
// ie dans Cargo.toml :
// gtk4 = { version = "0.11.5", features = ["v4_18"] }
//
use gtk4::{prelude::*, glib, gdk, gio};
use std::cell::RefCell;
//
// la fonction CssProvider::new() n'est pas une fonction const
// l'astuce consiste à initialiser un RefCell contenant 
// Option<CssProvider> avec la valeur None !!!
thread_local! {
	static CSS_PROVIDER: RefCell<Option<gtk4::CssProvider>> = 
		const { RefCell::new(None) };
}

fn main() {
    let app = gtk4::Application::builder()
        .application_id("com.exemple.colordialog")
        .build();
    app.connect_startup(start_ui);
    app.connect_activate(build_ui);
    app.run();
}

fn start_ui(_: &gtk4::Application) {
}

fn build_ui(app: &gtk4::Application) {
	// CSS
	moncss("rgb(0 255 0)");
    // Création du bouton qui va déclencher le dialogue
    let button = gtk4::Button::builder()
		.label("Choisir une couleur")
		.name("envert")
		.css_classes(["circular","mybutton",])
		.halign(gtk4::Align::Center)
		.build();
    //
    let espace = gtk4::Label::builder()
		.vexpand(true)
		.build();
	//
	let vbox = gtk4::Box::new(gtk4::Orientation::Vertical, 5);
	vbox.append(&button);
	vbox.append(&espace);
    // 3. Création de la fenêtre principale
    let window = gtk4::ApplicationWindow::builder()
		.application(app)
        .title("Exemple ColorDialog")
        .default_width(300)
        .default_height(200)
        .child(&vbox)
        .build();
    // Gestion du clic sur le bouton
    button.connect_clicked(glib::clone!(#[weak] window, move |_| {
		// Initialisation du ColorDialog
		let color_dialog = gtk4::ColorDialog::new();
		// ou RGBA::new(0f32, 0f32, 1f32, 1f32)
		let rgba = gdk::RGBA::BLUE;
		//
        color_dialog.choose_rgba(
            Some(&window),
            Some(&rgba),
            None::<&gio::Cancellable>,
            move |result| {
                // Le callback reçoit le résultat de la sélection
                let r = result.unwrap();
                let rr = format!("rgb({} {} {})", 
					(r.red()*255.0) as i32, 
                    (r.green()*255.0) as i32, 
                    (r.blue()*255.0) as i32,
                );
                moncss(&rr);
            }
        );
    }));
    // Affichage fenêtre
    window.present();
}

fn moncss(couleur: &str) {
    CSS_PROVIDER.with(|cell| {
        let mut serveur_opt = cell.borrow_mut();
        // Si n'existe pas encore pour ce thread, on le crée
        let provider = serveur_opt.get_or_insert_with(|| {
            let p = gtk4::CssProvider::new();
            if let Some(_display) = gtk4::gdk::Display::default() {
                gtk4::style_context_add_provider_for_display(
					&gdk::Display::default().unwrap(),
					&p,
					gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }
            p
        });
        // charge/met à jour le CSS (compatible GTK 4)
		let c = format!("
			.mybutton {{ padding : 0px 20px;}} 
			#envert {{ color: {}; }}
			",
			couleur 
		);
	provider.load_from_string(&c);
    });
}
