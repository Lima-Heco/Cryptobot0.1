pub mod ihm {

	extern crate gtk;
	extern crate cairo;
	use gtk::cairo as other_cairo;
	use gtk::prelude::*;
	use gtk::{Window, WindowType, DrawingArea};
	use crate::get_bitcoin::btcprice;
	
	use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};

	struct windowdat{
		x: f64,
		y: f64,
	}

	pub fn fenetre(btc_price: Arc<RwLock<btcprice>>) {

		let data_window = windowdat{x: 1000.0, y: 600.0};
		// Initialise GTK
		gtk::init().expect("Failed to initialize GTK.");

		// Créer une nouvelle fenêtre
		let window = Window::new(WindowType::Toplevel);
		window.set_title("Ma première fenêtre GTK");
		window.set_default_size(data_window.x.round() as i32, data_window.y.round() as i32);

		// Définir le comportement de fermeture de la fenêtre
		window.connect_delete_event(|_, _| {
			gtk::main_quit();

			Inhibit(false)
			
		});

		let drawing_area = DrawingArea::new();
    	window.add(&drawing_area);

		drawing_area.connect_draw(move |_, context| {
			let btc_price = Arc::clone(&btc_price);
			draw_candlesticks(context, &data_window, btc_price);
			Inhibit(false)
		});
		
		drawing_area.queue_draw();
		// Afficher la fenêtre
		window.show_all();

		// Exécuter la boucle principale de GTK
		gtk::main();
	}

	fn draw_candlesticks(context: &gtk::cairo::Context, window: &windowdat, btc_price: Arc<RwLock<btcprice>>) {
		// Exemple de données de chandelles
		let mut candlestick_data: Vec<[f64; 4]> = Vec::new();
		if true {
			let btc_price = Arc::clone(&btc_price);
			let price_guard = btc_price.read().unwrap();
			candlestick_data = price_guard.chandelle.clone();
		}
	
		let candle_width = 20.0; // Largeur d'une chandelle
		let candle_spacing = 10.0; // Espacement entre les chandelles
	
		let mut x = window.x / 4.0 * 3.0; // Position x de la première chandelle
		//let mut y = window.y / 2;
		for candle in candlestick_data {
			// Dessiner la chandelle
			draw_candlestick(context, x, candle, candle_width);
	
			// Déplacer la position x pour la prochaine chandelle
			x -= candle_width + candle_spacing;
		}
	}
	
	fn draw_candlestick(context: &gtk::cairo::Context, x: f64, candle: [f64; 4], width: f64) {
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
		context.rectangle(x, candle_top, width, candle_bottom - candle_top);
		context.fill();
	
		// Dessiner les mèches de la chandelle
		let wick_color = if close >= open { (0.0, 1.0, 0.0) } else { (1.0, 0.0, 0.0) }; // Couleur noire pour les mèches
		context.set_source_rgb(wick_color.0, wick_color.1, wick_color.2);
		context.move_to(x + width / 2.0, candle_top);
		context.line_to(x + width / 2.0, high);
		context.move_to(x + width / 2.0, candle_bottom);
		context.line_to(x + width / 2.0, low);
		context.stroke();
	}
}