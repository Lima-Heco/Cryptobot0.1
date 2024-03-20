pub mod BFC_1_brain {
    use crate::Data::get_btc::get_bitcoin; 
    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};

    pub fn bfcbrain(btc_price: Arc<RwLock<get_bitcoin::btcprice>>, should_stop_clone: Arc<Mutex<bool>>) {
        println!("bot started...");
        loop {
            
            if *should_stop_clone.lock().unwrap() {
                break;
            }
        }
        println!("bot finish...");
    }
}