pub mod BFC_1_brain {
    use crate::Data::get_btc::get_bitcoin;
    use crate::BFC_1::Bot_read::pente_view::tendance_view::BFC_1_view;
    use crate::BFC_1::Bot_mind::algo_1::BFC_1_mind::select_algo;
    use crate::BFC_1::Bot_mind::analyse::analyseBFC_1::marqueures;

    // /use crate::BFC_1::Bot_mind::algo_1;

    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};
    use crate::solde;

    pub fn bfcbrain(btc_price: Arc<RwLock<get_bitcoin::btcprice>>, should_stop_clone: Arc<Mutex<bool>>, infobot: Arc<Mutex<marqueures>>, selected: i32, mysold: Arc<Mutex<solde>>) {
        println!("bot started...");
        let mut view = BFC_1_view::new();
        let wait = Arc::new(Mutex::new(false));
        println!("Initialisation...");
//==========================================================INIT======================================================//
        loop {
            let btc_price_clone = Arc::clone(&btc_price);
            if view.init_new_slope(btc_price_clone) == 1 {
                break;
            }
        }
//==========================================================RECUP DATA===============================================//
        println!("Demmarage...");
        loop {
            let info = Arc::clone(&infobot);
            let btc_price_clone = Arc::clone(&btc_price);
            &view.get_potential(btc_price_clone);
            let btc_price_clone = Arc::clone(&btc_price);
            let mysold_clone = Arc::clone(&mysold);
            let wait_cl = Arc::clone(&wait);
            select_algo(&view, btc_price_clone, wait_cl, info, selected, mysold_clone);
            if *should_stop_clone.lock().unwrap() {
                break;
            }
        }
        println!("bot finish...");
    }
}