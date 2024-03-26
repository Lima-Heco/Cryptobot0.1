pub mod BFC_1_mind {
	use crate::BFC_1::Bot_read::pente_view::tendance_view::BFC_1_view;
    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};
	use crate::get_bitcoin;
	use crate::marqueures;

	pub fn select_algo(view: &BFC_1_view, btc_price: Arc<RwLock<get_bitcoin::btcprice>>, wait: Arc<Mutex<bool>>, infobot: Arc<Mutex<marqueures>>, selected: i32)
	{
//--------------------------------------------------------------------------------------
		if selected == 1 {
			let wait_cl = Arc::clone(&wait);
			let btc_price_clone = Arc::clone(&btc_price);
			let info = Arc::clone(&infobot);
			findV2(view, btc_price_clone, wait_cl, info);
		}
//--------------------------------------------------------------------------------------
		if selected == 2 {
			let wait_cl = Arc::clone(&wait);
			let btc_price_clone = Arc::clone(&btc_price);
			let info = Arc::clone(&infobot);
			findIVP2(view, btc_price_clone, wait_cl, info);
		}
//--------------------------------------------------------------------------------------
		if selected == 3 {
			let wait_cl = Arc::clone(&wait);
			let btc_price_clone = Arc::clone(&btc_price);
			let info = Arc::clone(&infobot);
			findIVP1(view, btc_price_clone, wait_cl, info);
		}
//---------------------------------------------------------------------------------------
		if selected == 4 {
			let wait_cl = Arc::clone(&wait);
			let btc_price_clone = Arc::clone(&btc_price);
			let info = Arc::clone(&infobot);
			let test = findP1(view, btc_price_clone, wait_cl, info);
		}
	}

	
	pub fn findV2(view: &BFC_1_view, btc_price: Arc<RwLock<get_bitcoin::btcprice>>, wait: Arc<Mutex<bool>>, infobot: Arc<Mutex<marqueures>>) -> i64{
		let wait_cl = Arc::clone(&wait);
		let btc_price_clone = Arc::clone(&btc_price);
		let info = Arc::clone(&infobot);
		if view.tableau[1].valeure_de_pente > 0.0 && view.tableau[0].valeure_de_pente < -35.0 && view.tableau[0].size > 4 && view.temp.valeure_de_pente > 25.0 && view.temp.size >= 3 && *wait_cl.lock().unwrap() == false && view.tableau[1].valeure_de_pente < 25.0{
			println!("________________________ZEEEEEEPARTIIIIII___________________________");
			let handle = thread::spawn(move || {
				let wait_cl = Arc::clone(&wait);
				let info = Arc::clone(&infobot);
				let mut inf = info.lock().unwrap();
				inf.findv2_try += 1;
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
                        println!("\n\n\n\n\nloose...\n\n\n\n\n\n\n");
                    } else {
						let info = Arc::clone(&infobot);
                        println!("\n\n\n\n\nwin...\n\n\n\n\n\n\n");
						inf.findv2_succes += 1;
                    }
                }
				let wait_cl = Arc::clone(&wait);
				*wait_cl.lock().unwrap() = false;
			});
			thread::sleep(time::Duration::from_secs(1));
			return 1;
		} 
		return 0;
	}
	

//_____________________________________________________________________________________________________________________________________


	pub fn findIVP1(view: &BFC_1_view, btc_price: Arc<RwLock<get_bitcoin::btcprice>>, wait: Arc<Mutex<bool>>, infobot: Arc<Mutex<marqueures>>) -> i64 {
		let wait_cl = Arc::clone(&wait);
		let info = Arc::clone(&infobot);
		let btc_price_clone = Arc::clone(&btc_price);
		if view.tableau[0].valeure_de_pente as f64 * view.tableau[0].size as f64 > -160.0
						&& view.tableau[0].size > 6 && view.tableau[0].size < 18
						&& view.temp.valeure_de_pente < 0.0
						&& *wait_cl.lock().unwrap() == false {
			println!("________________________ZEEEEEEPARTIIIIII___________________________");
			let handle = thread::spawn(move || {
				let wait_cl = Arc::clone(&wait);
				let info = Arc::clone(&infobot);
				*wait_cl.lock().unwrap() = true;
				let mut inf = info.lock().unwrap();
				inf.findivp1_try += 1;
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
                    if price < price_guard.btctousd {
                        println!("\n\n\n\n\nloose...\n\n\n\n\n\n\n");
                    } else {
						let info = Arc::clone(&infobot);
                        println!("\n\n\n\n\nwin...\n\n\n\n\n\n\n");
						inf.findivp1_succes += 1;
                    }
                }
				let wait_cl = Arc::clone(&wait);
				*wait_cl.lock().unwrap() = false;
			});
			thread::sleep(time::Duration::from_secs(1));
			return 1;
		} 
		return 0;
	}

	//_____________________________________________________________________________________________________________________________________


	pub fn findIVP2(view: &BFC_1_view, btc_price: Arc<RwLock<get_bitcoin::btcprice>>, wait: Arc<Mutex<bool>>, infobot: Arc<Mutex<marqueures>>) -> i64 {
		let wait_cl = Arc::clone(&wait);
		let info = Arc::clone(&infobot);
		let btc_price_clone = Arc::clone(&btc_price);
		if view.tableau[0].valeure_de_pente as f64 * view.tableau[0].size as f64 > -160.0
						&& view.tableau[0].size > 5 && view.tableau[0].size < 18
						&& view.temp.valeure_de_pente < 0.0
						&& view.tableau[1].valeure_de_pente > 0.0 && view.tableau[1].size > 4 && view.tableau[1].size < 7
						&& view.tableau[2].valeure_de_pente < 0.0 && view.tableau[2].size > 6
						&& *wait_cl.lock().unwrap() == false {
			println!("________________________ZEEEEEEPARTIIIIII___________________________");
			let handle = thread::spawn(move || {
				let wait_cl = Arc::clone(&wait);
				let info = Arc::clone(&infobot);
				*wait_cl.lock().unwrap() = true;
				let mut inf = info.lock().unwrap();
				inf.findivp2_try += 1;
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
                    if price < price_guard.btctousd {
                        println!("\n\n\n\n\nloose...\n\n\n\n\n\n\n");
                    } else {
						let info = Arc::clone(&infobot);
                        println!("\n\n\n\n\nwin...\n\n\n\n\n\n\n");
						inf.findivp2_succes += 1;
                    }
                }
				let wait_cl = Arc::clone(&wait);
				*wait_cl.lock().unwrap() = false;
			});
			thread::sleep(time::Duration::from_secs(1));
			return 1;
		} 
		return 0;
	}
	pub fn findP1(view: &BFC_1_view, btc_price: Arc<RwLock<get_bitcoin::btcprice>>, wait: Arc<Mutex<bool>>, infobot: Arc<Mutex<marqueures>>) -> i64 {
		let wait_cl = Arc::clone(&wait);
		let info = Arc::clone(&infobot);
		let btc_price_clone = Arc::clone(&btc_price);
		if view.tableau[0].valeure_de_pente as f64 * view.tableau[0].size as f64 > 150.0 && view.tableau[0].size > 4 && view.tableau[0].size < 7 && *wait_cl.lock().unwrap() == false {
			println!("________________________ZEEEEEEPARTIIIIII___________________________");
			let handle = thread::spawn(move || {
				let wait_cl = Arc::clone(&wait);
				let info = Arc::clone(&infobot);
				let mut inf = info.lock().unwrap();
				inf.findp1_try += 1;
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
                        println!("\n\n\n\n\nloose...\n\n\n\n\n\n\n");
                    } else {
						let info = Arc::clone(&infobot);
                        println!("\n\n\n\n\nwin...\n\n\n\n\n\n\n");
						inf.findp1_succes += 1;
                    }
                }
				let wait_cl = Arc::clone(&wait);
				*wait_cl.lock().unwrap() = false;
			});
			thread::sleep(time::Duration::from_secs(1));
			return 1;
		} 
		return 0;
	}
}
