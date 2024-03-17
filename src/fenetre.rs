pub mod ihm {

	extern crate gtk;
	use gtk::prelude::*;
	use gtk::{Window, WindowType};

	pub fn fenetre() {
		// Initialise GTK
		gtk::init().expect("Failed to initialize GTK.");

		// Créer une nouvelle fenêtre
		let window = Window::new(WindowType::Toplevel);
		window.set_title("Ma première fenêtre GTK");
		window.set_default_size(400, 300);

		// Définir le comportement de fermeture de la fenêtre
		window.connect_delete_event(|_, _| {
			gtk::main_quit();
			Inhibit(false)
		});

		// Afficher la fenêtre
		window.show_all();

		// Exécuter la boucle principale de GTK
		gtk::main();
	}
}