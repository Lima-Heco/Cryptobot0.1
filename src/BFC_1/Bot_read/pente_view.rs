pub mod tendance_view {
    use crate::BFC_1::Bot_read::pente::tendances::pente;
    use crate::Data::get_btc::get_bitcoin; 
    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};

    pub struct BFC_1_view {
        tableau: [pente; 3],
        temp: pente,
        next_time: bool,
    }
    
    impl BFC_1_view {
        pub fn new() -> Self {
            BFC_1_view {
                tableau: [
                    pente::new(),
                    pente::new(),
                    pente::new(),
                ],
                temp: pente::new(),
                next_time: true,
            }
        }
        pub fn init_new_slope(self: &mut Self, btc_price: Arc<RwLock<get_bitcoin::btcprice>>) -> i32{
            let btc_price_clone = Arc::clone(&btc_price);
            let price_guard = btc_price_clone.read().unwrap();
            if self.tableau[0].initialized == false {
                self.tableau[0].start_timestamp = price_guard.timestamp;
                self.tableau[0].start_price = price_guard.btctousd;
                self.tableau[0].initialized = true;
                self.tableau[0].size += 1;
                return -1;
            }
            if self.tableau[0].size == 1 && self.next_time != price_guard.next_time {
                self.tableau[0].end_timestamp = price_guard.timestamp;
                self.tableau[0].end_price = price_guard.btctousd;
                self.tableau[0].size += 1;
                self.tableau[0].valeure_de_pente = pente::calculate_slope(self.tableau[0].start_timestamp, self.tableau[0].start_price, self.tableau[0].end_timestamp, self.tableau[0].end_price);
                println!("initialized");
                return 1;
            }
            return -1;
        }
        //estime la pente potentielle apartire de la derniere. si elle est egale a l'ancienne reinitialisation et actualisation de l'ancienne
        pub fn get_potential() {
            
        }
    }
}