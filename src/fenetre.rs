pub mod ihm {

	extern crate gtk;
	extern crate cairo;
	use gtk::prelude::*;
	use gtk::{Window, WindowType, DrawingArea};

	struct windowdat{
		x: f64,
		y: f64,
	}

	/*fn put_price_s(context: &cairo::Context, window: &windowdat) {
			// Dessiner des pixels ici
		context.set_source_rgb(0.0, 1.0, 0.0); // Couleur rouge
		context.rectangle((window.x - 100.0), (window.y / 2.0), 10.0, 1.0); // Rectangle de 10x10 pixels
		context.fill(); // Remplir le rectangle
	}*/

	pub fn fenetre() {

		/*let data_window = windowdat{x: 1000.0, y: 600.0};
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

		drawing_area.connect_draw(|_, context| {
			// Appelez la fonction de dessin du rectangle
			put_price_s(context, &data_window);
	
			Inhibit(false)
		});

		// Afficher la fenêtre
		window.show_all();

		// Exécuter la boucle principale de GTK
		gtk::main();*/
	}
}