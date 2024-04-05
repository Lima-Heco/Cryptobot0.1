pub mod ihm {

	extern crate gtk;
	extern crate cairo;
	//use gtk::rgba::RGBA;
	use gtk::cairo as other_cairo;
	use std::time::Duration;
	use glib::timeout_add;
	use gtk::prelude::*;
	use gtk::{Window, WindowType, DrawingArea, Button, Label};
	use glib::signal::Propagation;
	use glib::ControlFlow::Continue;
	use crate::get_bitcoin::btcprice;
	use crate::solde;
	use crate::BFC_1_brain;
	use crate::BFC_1::Bot_mind::analyse::analyseBFC_1::marqueures;
	
	use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};

	const PURPLE_COLOR: (f64, f64, f64) = (0.5, 0.0, 0.5);

	struct windowdat{
		x: f64,
		y: f64,
	}

	pub fn fenetre(btc_price: Arc<RwLock<btcprice>>, mysold: Arc<Mutex<solde>>, should_stop_clone: Arc<Mutex<bool>>, infobot: Arc<Mutex<marqueures>>) {

		let data_window = windowdat{x: 1920.0, y: 990.0};
		// Initialise GTK
		gtk::init().expect("Failed to initialize GTK.");

		// Créer une nouvelle fenêtre
		let window = Window::new(WindowType::Toplevel);
		window.set_title("Cryptobot");
		window.set_default_size(data_window.x.round() as i32, data_window.y.round() as i32);

		let mut thesolde: f64 = 0.0;

		if true {
			let mysold_clone = Arc::clone(&mysold);
			let mut onlinesold = mysold_clone.lock().unwrap();
			thesolde = onlinesold.solde;
		}

		let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    	window.add(&main_box);
		// Créer un label avec le chiffre à afficher
		
		let menu_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
		main_box.pack_end(&menu_box, false, false, 0);
		
		let label_text = format!("Solde: {}", thesolde);
		let price_label = Label::new(Some(&label_text)); // Chiffre à afficher
		price_label.set_margin_bottom(150);
    	price_label.set_margin_end(150);
		menu_box.pack_end(&price_label, false, false, 0);

		let button_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
		menu_box.pack_start(&button_box, false, false, 0);
		//window.add(&button_box);
		let cloned_window = window.clone();
		let button_refresh = Button::with_label("Refresh Window");
		button_refresh.set_margin_end(100);
		button_refresh.set_margin_top(20);
		button_refresh.set_size_request(200, 50);
		button_refresh.connect_clicked(move |_| {
			// Resize the window to a new size
			
			cloned_window.resize(600, 400);
			
		});
		let btc_price_clone = Arc::clone(&btc_price);
		let button_bot = Button::with_label("go down bot");
		button_bot.set_margin_end(100);
		button_bot.set_margin_top(20);
		button_bot.set_size_request(200, 50);
		button_bot.connect_clicked(move |_| {
			let btc_price_clone = Arc::clone(&btc_price_clone);
				let should_stop_bot = Arc::clone(&should_stop_clone);
                let mysolde_clone = Arc::clone(&mysold);
                let info2 = Arc::clone(&infobot);
                let handle = thread::spawn(move || {
					
                    BFC_1_brain::bfcbrain(btc_price_clone, should_stop_bot, info2, 5, mysolde_clone);
                });
                thread::sleep(time::Duration::from_secs(2));
		});
		button_box.pack_start(&button_refresh, false, false, 0);
		button_box.pack_start(&button_bot, false, false, 0);

		let drawing_area = DrawingArea::new();
		drawing_area.set_size_request(1500, 990); // Taille de la zone de dessin
		main_box.pack_start(&drawing_area, true, true, 0);

		// Clonage de la zone de dessin pour utilisation dans la fermeture
		let drawing_area_clone = drawing_area.clone();

		// Connexion de la fonction de rappel à l'événement de redimensionnement de la fenêtre
		window.connect_configure_event(move |_, _| {
			// Actualisation du contenu de la zone de dessin
			drawing_area_clone.queue_draw();
			let test = Propagation::from(false);
			test.into()
		});
		//window.add(&drawing_area);
		drawing_area.connect_draw(move |_, context| {

			// Définition de l'épaisseur de la bordure
			let x = 20.0;
			let y = 20.0;
			let width = 1500.0;
			let height = 970.0;
			let radius = 20.0; // Rayon pour arrondir les coins
			
			context.move_to(x + radius, y); // Déplacer le point de départ au coin supérieur gauche arrondi
			context.arc(x + width - radius, y + radius, radius, -90.0_f64.to_radians(), 0.0_f64.to_radians()); // Dessiner l'arc supérieur droit
			context.arc(x + width - radius, y + height - radius, radius, 0.0_f64.to_radians(), 90.0_f64.to_radians()); // Dessiner l'arc inférieur droit
			context.arc(x + radius, y + height - radius, radius, 90.0_f64.to_radians(), 180.0_f64.to_radians()); // Dessiner l'arc inférieur gauche
			context.arc(x + radius, y + radius, radius, 180.0_f64.to_radians(), 270.0_f64.to_radians()); // Dessiner l'arc supérieur gauche
			context.close_path(); // Fermer le chemin

			context.set_line_width(5.0);

			// Dessin du rectangle de la bordure
			context.set_source_rgb(PURPLE_COLOR.0, PURPLE_COLOR.1, PURPLE_COLOR.2);
			//context.rectangle(0.0, 0.0, 1500.0, 1040.0);
			context.stroke();
			let btc_price = Arc::clone(&btc_price);
			draw_candlesticks(context, &data_window, btc_price);
			//Inhibit(false)
			let test = Propagation::from(false);
			test
		});
		
		add_drawing_to_queue(&drawing_area);
		fn add_drawing_to_queue(drawing_area: &DrawingArea) {
			drawing_area.queue_draw();
		}
	
		// Exemple d'appel de la fonction pour ajouter un nouveau dessin à la file d'attente
		// Vous pouvez appeler cette fonction chaque fois que vous avez un nouveau dessin à ajouter

		let drawing_area_clone = drawing_area.clone();
		// Définir le comportement de fermeture de la fenêtre
		window.connect_delete_event(move |_, _| {
			gtk::main_quit();
			drawing_area_clone.queue_draw();
			//Inhibit(false)
			let test = Propagation::from(false);
			test
		});
	
		
		drawing_area.queue_draw();
		// Afficher la fenêtre
		window.show_all();

		let drawing_area_clone = drawing_area.clone();

		window.connect_size_allocate(move |_, _| {
			drawing_area_clone.queue_draw();
		});

		//simulate_window_resize(&window, &drawing_area, Duration::from_millis(500));

		// Exécuter la boucle principale de GTK
		gtk::main();
	}

	fn draw_candlesticks(context: &gtk::cairo::Context, window: &windowdat, btc_price: Arc<RwLock<btcprice>>) {
		// Exemple de données de chandelles
		let mut candlestick_data: Vec<[f64; 4]> = Vec::new();
		let mut points: Vec<[f64; 2]> = Vec::new();
		let mut firstchandel: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
		if true {
			let btc_price = Arc::clone(&btc_price);
			let price_guard = btc_price.read().unwrap();
			points = price_guard.point.clone();
		}
		draw_line_chart(context, &points, 1500.0, 990.0)
	}
	
	fn draw_candlestick(context: &gtk::cairo::Context, x: f64, candle: [f64; 4], firstcandle: [f64; 4], width: f64) {
		let open = candle[0];
		let high = candle[1];
		let low = candle[2];
		let close = candle[3];
		let candle_top = high;
		let candle_bottom = low.min(open).min(close);
		let candle_height = high - low;
	
		// Dessiner le corps de la chandelle
		let body_color = if close >= open { (0.0, 1.0, 0.0) } else { (1.0, 0.0, 0.0) }; // Vert si hausse, rouge sinon
		context.set_source_rgb(body_color.0, body_color.1, body_color.2);
		if close >= open {
			context.rectangle(x, close - firstcandle[0] + (970.0 / 2.0), width, open - firstcandle[0] + (970.0 / 2.0));
		} else {
			context.rectangle(x, close - firstcandle[0] + (970.0 / 2.0), width, open - firstcandle[0] + (970.0 / 2.0));
		}
		context.fill();
	
		// Dessiner les mèches de la chandelle
		let wick_color = (0.0, 0.0, 0.0); // Couleur noire pour les mèches
		context.set_source_rgb(wick_color.0, wick_color.1, wick_color.2);
		context.move_to(x + width / 2.0, candle_top);
		context.line_to(x + width / 2.0, high);
		context.move_to(x + width / 2.0, candle_bottom);
		context.line_to(x + width / 2.0, low);
		context.stroke();
	}

	fn draw_line_chart(
    context: &gtk::cairo::Context,
    points: &Vec<[f64; 2]>, // Vecteur de points [x, y]
    window_width: f64,
    window_height: f64,
) {
    // Vérifier s'il y a suffisamment de points pour dessiner un graphique
    if points.len() < 2 {
        return;
    }

    // Déterminer le maximum et le minimum des valeurs x et y
    let (min_x, max_x) = points.iter().map(|p| p[0]).fold((std::f64::INFINITY, std::f64::NEG_INFINITY), |(min, max), x| (min.min(x), max.max(x)));
    let (min_y, max_y) = points.iter().map(|p| p[1]).fold((std::f64::INFINITY, std::f64::NEG_INFINITY), |(min, max), y| (min.min(y), max.max(y)));

    // Calculer les facteurs d'échelle pour ajuster les points au maximum et au minimum de la fenêtre
    let scale_x = (window_width - 60.0) / (max_x - min_x); // Soustraire 60.0 pour la marge de 30 pixels des deux côtés
    let scale_y = (window_height - 60.0) / (max_y - min_y); // Soustraire 60.0 pour la marge de 30 pixels des deux côtés

    // Dessiner les lignes entre les points
    let mut previous_point = points[0];
    for point in points.iter().skip(1) {
        let x = (point[0] - min_x) * scale_x + 40.0;
        let y = window_height - ((point[1] - min_y) * scale_y + 40.0);
        let direction = if point[1] > previous_point[1] { // Si la ligne est vers le haut
            context.set_source_rgb(0.0, 1.0, 0.0); // Vert
        } else { // Si la ligne est vers le bas
            context.set_source_rgb(1.0, 0.0, 0.0); // Rouge
        };
        context.move_to((previous_point[0] - min_x) * scale_x + 40.0, window_height - ((previous_point[1] - min_y) * scale_y + 40.0));
        context.line_to(x, y);
        context.stroke();
        previous_point = *point;
    }

    // Dessiner l'échelle x en bas
    let x_scale_step = (max_x - min_x) / 10.0; // Diviser l'intervalle x en 10 étapes
    for i in 0..=10 {
        let x = 40.0 + i as f64 * (window_width - 80.0) / 10.0; // Soustraire 80.0 pour une marge de 40 pixels des deux côtés
        let y1 = window_height - 40.0;
        let y2 = window_height - 50.0; // Ajouter une marge de 10 pixels entre l'échelle et le graphique
        context.move_to(x, y1);
        context.line_to(x, y2);
        context.stroke();

        // Ajouter des étiquettes pour l'échelle x
        let label = format!("{:.2}", min_x + i as f64 * x_scale_step);
        context.move_to(x - 10.0, y1 + 15.0);
        context.show_text(&label);
    }

    // Dessiner l'échelle y à gauche
    let y_scale_step = (max_y - min_y) / 10.0; // Diviser l'intervalle y en 10 étapes
    for i in 0..=10 {
        let x1 = 40.0;
        let x2 = 50.0; // Ajouter une marge de 10 pixels entre l'échelle et le graphique
        let y = window_height - 40.0 - i as f64 * (window_height - 80.0) / 10.0; // Soustraire 80.0 pour une marge de 40 pixels des deux côtés
        context.move_to(x1, y);
        context.line_to(x2, y);
        context.stroke();

        // Ajouter des étiquettes pour l'échelle y
        let label = format!("{:.2}", min_y + i as f64 * y_scale_step);
        context.move_to(x1 - 30.0, y + 5.0);
        context.show_text(&label);
    }

    // Placer la dernière occurrence ajoutée à deux tiers de la bordure droite de la fenêtre
    let last_point = points.last().unwrap();
    let last_x = (last_point[0] - min_x) * scale_x + 40.0;
    let last_y = window_height - ((last_point[1] - min_y) * scale_y + 40.0);
    let x_offset = (window_width - 80.0) / 3.0 * 2.0; // Soustraire 80.0 pour une marge de 40 pixels des deux côtés
    context.move_to(last_x + x_offset, last_y);

    // Définir la couleur de la ligne à blanc
    context.set_source_rgb(1.0, 1.0, 1.0); // Blanc
    
    // Définir l'épaisseur de la ligne
    context.set_line_width(1.0);
    
    // Dessiner la ligne
    context.stroke();
}

}