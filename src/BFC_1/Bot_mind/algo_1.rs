pub mod BFC_1_mind {
	use crate::BFC_1::Bot_read::pente_view::tendance_view::BFC_1_view;
    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};
	use crate::get_bitcoin;

	pub fn findV1(view: &BFC_1_view, btc_price: Arc<RwLock<get_bitcoin::btcprice>>, wait: Arc<Mutex<bool>>){
		let wait_cl = Arc::clone(&wait);
		let btc_price_clone = Arc::clone(&btc_price);
		if view.tableau[1].valeure_de_pente > 0.0 && view.tableau[0].valeure_de_pente < -20.0 && view.tableau[0].valeure_de_pente > -35.0 && view.tableau[0].size > 3 && view.temp.valeure_de_pente > 20.0 && view.temp.size >= 2 && *wait_cl.lock().unwrap() == false{
			println!("________________________ZEEEEEEPARTIIIIII___________________________");
			let handle = thread::spawn(move || {
				let wait_cl = Arc::clone(&wait);
				*wait_cl.lock().unwrap() = true;
				let mut price = 0.0;
                if true {
                    let btc_price_clone = Arc::clone(&btc_price);
                    let price_guard = btc_price_clone.read().unwrap();
                    price = price_guard.btctousd;
                }
                thread::sleep(time::Duration::from_secs(90));
                if true {
                    let btc_price_clone = Arc::clone(&btc_price);
                    let price_guard = btc_price_clone.read().unwrap();
                    if price > price_guard.btctousd {
                        println!("\n\n\n\n\nloose...\n\n\n\n\n\n\n")
                    } else {
                        println!("\n\n\n\n\nwin...\n\n\n\n\n\n\n")
                    }
                }
				let wait_cl = Arc::clone(&wait);
				*wait_cl.lock().unwrap() = false;
			});
			thread::sleep(time::Duration::from_secs(1));
		} else if view.tableau[1].valeure_de_pente > 0.0 && view.tableau[0].valeure_de_pente < -35.0 && view.tableau[0].size > 4 && view.temp.valeure_de_pente > 25.0 && view.temp.size >= 3 && *wait_cl.lock().unwrap() == false{
			println!("________________________ZEEEEEEPARTIIIIII___________________________");
			let handle = thread::spawn(move || {
				let wait_cl = Arc::clone(&wait);
				*wait_cl.lock().unwrap() = true;
				let mut price = 0.0;
				if true {
					let btc_price_clone = Arc::clone(&btc_price);
					let price_guard = btc_price_clone.read().unwrap();
					price = price_guard.btctousd;
				}
				thread::sleep(time::Duration::from_secs(90));
				if true {
					let btc_price_clone = Arc::clone(&btc_price);
					let price_guard = btc_price_clone.read().unwrap();
					if price > price_guard.btctousd {
						println!("\n\n\n\n\nloose...\n\n\n\n\n\n\n")
					} else {
						println!("\n\n\n\n\nwin...\n\n\n\n\n\n\n")
					}
				}
				let wait_cl = Arc::clone(&wait);
				*wait_cl.lock().unwrap() = false;
			});
			thread::sleep(time::Duration::from_secs(1));
		}
	} 
}