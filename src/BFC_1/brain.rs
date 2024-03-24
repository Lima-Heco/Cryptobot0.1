pub mod BFC_1_brain {
    use crate::Data::get_btc::get_bitcoin;
    use crate::BFC_1::Bot_read::pente_view::tendance_view::BFC_1_view;

    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};

    pub fn bfcbrain(btc_price: Arc<RwLock<get_bitcoin::btcprice>>, should_stop_clone: Arc<Mutex<bool>>) {
        println!("bot started...");
        let mut view = BFC_1_view::new();
        println!("Initialisation...");
        loop {
            let btc_price_clone = Arc::clone(&btc_price);
            if view.init_new_slope(btc_price_clone) == 1 {
                break;
            }
        }
        println!("Demmarage...");
        loop {
            let btc_price_clone = Arc::clone(&btc_price);
            view.get_potential(btc_price_clone);
            if *should_stop_clone.lock().unwrap() {
                break;
            }
        }
        println!("bot finish...");
    }
}