use chrono::Local;
use gtk::{gdk, gio, glib, prelude::*};
use std::cell::Cell;
use std::f64::consts::PI;
use std::rc::Rc;

const APP_ID: &str = "org.gtk_rs.Actions1";

const DEFINITION_CSS: &str = "
	frame {border: double 3px lightblue;}
	frame label {color: red;}
	.drawingarea {border-radius: 8px; border: double 3px lightblue;}
";

fn main() -> gtk::glib::ExitCode {
    // Create a new application
    let app = gtk::Application::builder().application_id(APP_ID).build();
    // css
    app.connect_startup(|app| {
        let provider = gtk::CssProvider::new();
        provider.load_from_data(DEFINITION_CSS);
        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
        //
        // menu
        //
        let about_menu_item = gio::MenuItem::new(Some("Change bouton"), Some("app.rouge"));
        let quit_menu_item = gio::MenuItem::new(
            Some("Quitter"),
            Some("app.quitter"), // action quit
        );
        let rouge_item = gio::MenuItem::new(Some("Rouge"), Some("app.rouge"));
        //
        let bleu_item = gio::MenuItem::new(Some("Bleu"), Some("app.bleu"));
        //
        let file_menu = gio::Menu::new();
        file_menu.append_item(&about_menu_item);
        file_menu.append_item(&quit_menu_item);
        //
        let couleur_menu = gio::Menu::new();
        couleur_menu.append_item(&rouge_item);
        couleur_menu.append_item(&bleu_item);
        //
        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("Fichiers"), &file_menu);
        menubar.append_submenu(Some("Couleurs"), &couleur_menu);
        app.set_menubar(Some(&menubar));
    });
    //
    app.connect_activate(build_ui);
    app.run() // Run the application
}

fn build_ui(app: &gtk::Application) {
    // définit la couleur de la courbe
    let rvb = Rc::new(Cell::new((1.0, 0.2, 0.2)));
    // définit position click souris : init 10 10
    let xy = Rc::new(Cell::new((10.0, 10.0, 0.0, 0.0, false)));
    // Create a window and set the title
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Demo Actions")
        .width_request(600) // largeur mini
        .height_request(400) // hauteur mini
        .show_menubar(true)
        .build();
    window.set_default_size(800, 600);
    //
    let label = gtk::Label::default();
    //
    let but1 = gtk::Button::with_label("Quitter");
    but1.set_hexpand(false);
    but1.set_vexpand(false);
    but1.set_halign(gtk::Align::Center);
    but1.set_valign(gtk::Align::Center);
    but1.add_css_class("destructive-action");
    //
    let header_bar = gtk::HeaderBar::new(); // objet barre titre
    window.set_titlebar(Some(&header_bar)); // assoc avec fenetre
    header_bar.pack_start(&but1);
    header_bar.pack_end(&label);
    let time = format!("{}", Local::now().format("%H:%M:%S"));
    label.set_text(&time);
    header_bar.set_decoration_layout(Some("")); // defaut : "menu:minimize,maximize,close"
                                                //
    let lab1 = gtk::Label::new(Some("  Ctrl + w\npour quitter"));
    //
    const M: i32 = 5;
    let fra1 = gtk::Frame::new(Some(""));
    fra1.set_margin_top(0);
    fra1.set_margin_bottom(0);
    fra1.set_margin_start(0);
    fra1.set_margin_end(M);
    fra1.set_child(Some(&lab1));
    //
    let infos = gtk::Label::builder()
        .hexpand(true)
        .halign(gtk::Align::Fill)
        .xalign(0.0)
        .single_line_mode(true)
        .label("Ctrl + w pour quitter")
        .build();
    //
    let courbes = gtk::DrawingArea::builder()
        .vexpand(true)
        .hexpand(true)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .margin_top(0)
        .margin_bottom(0)
        .margin_start(M)
        .margin_end(0)
        .build();
    courbes.add_css_class("drawingarea");
    //
    let box1 = gtk::Paned::builder()
        .orientation(gtk::Orientation::Horizontal)
        .hexpand(true)
        .vexpand(true)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .end_child(&courbes)
        .start_child(&fra1)
        .resize_start_child(false)
        .wide_handle(true)
        .position(100)
        .build();
    //
    let box2 = gtk::Box::builder()
        .hexpand(true)
        .vexpand(true)
        .halign(gtk::Align::Fill)
        .valign(gtk::Align::Fill)
        .orientation(gtk::Orientation::Vertical)
        .margin_top(M)
        .margin_bottom(M)
        .margin_start(M)
        .margin_end(M)
        .spacing(M)
        .build();
    box2.append(&box1);
    box2.append(&infos);
    //
    // gestures
    //
    let souris = gtk::GestureClick::new();
    souris.set_button(3u32);
    courbes.add_controller(souris.clone());
    //
    let bouge = gtk::GestureDrag::new();
    bouge.set_button(1u32);
    courbes.add_controller(bouge.clone());
    //
    // Add Actions
    //
    let quitter = gio::ActionEntry::builder("quitter")
        .activate(move |app: &gtk::Application, _, _| {
            app.quit();
        })
        .build();
    app.set_accels_for_action("app.quitter", &["<Ctrl>W"]);
    //
    let rouge = gio::ActionEntry::builder("rouge")
        .activate(glib::clone!(
            #[weak]
            courbes,
            #[weak]
            but1,
            #[strong]
            rvb,
            move |_, _, _| {
                but1.set_css_classes(&["destructive-action"]);
                rvb.set((1.0, 0.2, 0.2));
                courbes.queue_draw();
            }
        ))
        .build();
    //
    let bleu = gio::ActionEntry::builder("bleu")
        .activate(glib::clone!(
            #[weak]
            courbes,
            #[weak]
            but1,
            #[strong]
            rvb,
            move |_, _, _| {
                but1.set_css_classes(&["suggested-action"]);
                rvb.set((0.2, 0.2, 1.0));
                courbes.queue_draw();
            }
        ))
        .build();
    //
    app.add_action_entries([bleu, rouge, quitter]);
    //
    // Add Connects
    //
    souris.connect_released(glib::clone!(
        #[strong]
        xy,
        #[weak]
        courbes,
        move |_, _, xx, yy| {
            xy.set((xx, yy, 0.0, 0.0, false));
            courbes.queue_draw();
        }
    ));
    //
    bouge.connect_drag_begin(glib::clone!(
        #[strong]
        xy,
        move |_, xx, yy| {
            let (xs, ys, _, _, _) = xy.get();
            if ((xs - xx).abs() < 5.0) || ((ys - yy).abs() < 5.0) {
                xy.set((xx, yy, 0.0, 0.0, true));
            }
        }
    ));
    //
    bouge.connect_drag_update(glib::clone!(
        #[strong]
        xy,
        #[weak]
        courbes,
        move |_, xx, yy| {
            let (xs, ys, _, _, etat) = xy.get();
            if etat {
                xy.set((xs, ys, xx, yy, true));
                courbes.queue_draw();
            }
        }
    ));
    //
    bouge.connect_drag_end(glib::clone!(
        #[strong]
        xy,
        move |_, xx, yy| {
            let (xs, ys, _, _, etat) = xy.get();
            if etat {
                xy.set((xs + xx, ys + yy, 0.0, 0.0, false));
            }
        }
    ));
    //
    but1.connect_clicked(glib::clone!(
        #[weak]
        window,
        move |_| window.close()
    ));
    //
    // drawarea
    //
    courbes.set_draw_func(glib::clone!(
        #[strong]
        xy,
        move |_area, cx, w, h| {
            //
            let ww = f64::from(w);
            let hh = f64::from(h);
            // axes
            cx.set_source_rgb(0.3, 0.3, 1.0);
            // axe vertical
            cx.move_to(10.0, hh - 10.0);
            cx.line_to(10.0, 10.0);
            cx.line_to(5.0, 25.0);
            cx.move_to(10.0, 10.0);
            cx.line_to(15.0, 25.0);
            // axe horizontal
            cx.move_to(10.0, hh / 2.0);
            cx.line_to(ww - 10.0, hh / 2.0);
            cx.line_to(ww - 25.0, hh / 2.0 - 5.0);
            cx.move_to(ww - 10.0, hh / 2.0);
            cx.line_to(ww - 25.0, hh / 2.0 + 5.0);
            cx.stroke().unwrap();
            // texte
            cx.set_source_rgb(0.1, 1.0, 0.1);
            cx.move_to(ww - 120.0, hh - 5.0);
            cx.set_font_size(20.0);
            let info = format!("{}x{}", w, h);
            cx.show_text(&info).unwrap();
            cx.stroke().unwrap();
            // courbe
            let (r, v, b) = rvb.get();
            cx.set_source_rgb(r, v, b); // rouge
            cx.move_to(10.0, hh / 2.0);
            for x in (0..(w - 20)).map(|x| x as f64) {
                let xx = x * 4.0 * PI / (ww - 20.0);
                let yy = xx.sin();
                cx.line_to(x + 10.0, hh / 2.0 - yy * (hh / 2.0 - 10.0));
            }
            cx.stroke().unwrap();
            // echelle X = π
            cx.set_source_rgb(0.3, 0.3, 1.0);
            cx.move_to(10.0 + (ww - 20.0) / 4.0 - 4.0, hh / 2.0 + 25.0);
            cx.show_text(&"π").unwrap();
            cx.move_to(10.0 + (ww - 20.0) / 4.0, hh / 2.0 - 5.0);
            cx.line_to(10.0 + (ww - 20.0) / 4.0, hh / 2.0 + 5.0);
            cx.stroke().unwrap();
            // position click
            cx.set_source_rgb(0.2, 0.2, 0.2);
            let (xsouris, ysouris, dx, dy, _) = xy.get();
            cx.arc(xsouris + dx, ysouris + dy, 5.0, 0.0, 2.0 * PI);
            cx.stroke().unwrap();
        }
    ));
    //
    // Present window
    //
    window.set_child(Some(&box2));
    window.present();
    // we are using a closure to capture the label (else we could also use a normal
    // function)
    let tick = move || {
        let time = format!("{}", Local::now().format("%H:%M:%S"));
        label.set_text(&time);
        window.set_title(Some(&time));
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };
    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
}
