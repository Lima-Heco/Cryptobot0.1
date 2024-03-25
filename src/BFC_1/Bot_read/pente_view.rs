pub mod tendance_view {
    use crate::BFC_1::Bot_read::pente::tendances::pente;
    use crate::Data::get_btc::get_bitcoin; 
    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};
    use std::clone::Clone;

    pub struct BFC_1_view {
        pub tableau: [pente; 3],
        pub temp: pente,
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

        pub fn is_init(self: &mut Self) -> bool {
            true
        }

        pub fn swaptab(self: &mut Self) {
            let mut i = self.tableau.len() - 1;
            println!("size of tab : {}", i);
            while i != 0 {
                self.tableau[i] = self.tableau[i - 1].clone();
                i -= 1;
            }
            self.tableau[0] = self.temp.clone();
            self.temp = pente::new();
        }

        pub fn affiche_structure(self: &mut Self) {
            let mut temp = "0";
            if self.tableau[2].valeure_de_pente < 0.0 {
                temp = "\x1b[31m"
            } else {
                temp = "\x1b[32m"
            }
            println!("{} pente n3: \n depart: {}\n arrivee: {}\n taille: {}\n pente: {}\x1b[0m\n", temp, self.tableau[2].start_price, self.tableau[2].end_price, self.tableau[2].size, self.tableau[2].valeure_de_pente);
            if self.tableau[1].valeure_de_pente < 0.0 {
                temp = "\x1b[31m"
            } else {
                temp = "\x1b[32m"
            }
            println!("{} pente n2: \n depart: {}\n arrivee: {}\n taille: {}\n pente: {}\x1b[0m\n", temp, self.tableau[1].start_price, self.tableau[1].end_price, self.tableau[1].size, self.tableau[1].valeure_de_pente);
            if self.tableau[0].valeure_de_pente < 0.0 {
                temp = "\x1b[31m"
            } else {
                temp = "\x1b[32m"
            }
            println!("{} pente n1: \n depart: {}\n arrivee: {}\n taille: {}\n pente: {}\x1b[0m\n", temp, self.tableau[0].start_price, self.tableau[0].end_price, self.tableau[0].size, self.tableau[0].valeure_de_pente);
            if self.temp.valeure_de_pente < 0.0 {
                temp = "\x1b[31m"
            } else {
                temp = "\x1b[32m"
            }
            println!("{} pente n0: \n depart: {}\n arrivee: {}\n taille: {}\n pente: {}\x1b[0m\n", temp, self.temp.start_price, self.temp.end_price, self.temp.size, self.temp.valeure_de_pente);

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
                self.tableau[0].valeure_de_pente = pente::calculate_slope(0, self.tableau[0].start_price, self.tableau[0].size, self.tableau[0].end_price);
                println!("initialized");
                return 1;
            }
            return -1;
        }
        pub fn get_potential(self: &mut Self, btc_price: Arc<RwLock<get_bitcoin::btcprice>>) {
            let mut time = true;
            let mut cop = pente::new();
            if self.temp.initialized == false {
                self.temp.start_price = self.tableau[0].end_price;
                self.temp.start_timestamp = self.tableau[0].end_timestamp;
                self.temp.initialized = true;
            }
            if true {
                let btc_price_clone = Arc::clone(&btc_price);
                let price_guard = btc_price_clone.read().unwrap();
                self.temp.end_timestamp = price_guard.timestamp;
                self.temp.end_price = price_guard.btctousd;
                time = price_guard.next_time;
            }
            if self.temp.size == 0 {
                self.temp.size = 1;
            }
            self.temp.valeure_de_pente = pente::calculate_slope(0, self.temp.start_price, self.temp.size, self.temp.end_price);
            cop.init(0, self.tableau[0].start_price, self.temp.size + self.tableau[0].size - 1, self.temp.end_price, self.temp.size + self.tableau[0].size - 1);
            if self.next_time != time
            {
                self.temp.size += 1;
                let pourcentage_dacceptaition = self.tableau[0].valeure_de_pente * 10.0 / 100.0;
                if self.temp.valeure_de_pente > -6.0 && self.temp.valeure_de_pente < 6.0 && self.temp.size < 9 || (cop.valeure_de_pente > 0.0 && self.temp.valeure_de_pente < 0.0) && self.temp.size < 6
                        || (cop.valeure_de_pente > 0.0 && self.temp.valeure_de_pente < 0.0) && self.temp.size < 6 {
                    println!("wait");
                } else if cop.valeure_de_pente >= (self.tableau[0].valeure_de_pente - 30.0) && cop.valeure_de_pente <= (self.tableau[0].valeure_de_pente + 30.0) {
                    if self.tableau[0].valeure_de_pente < 0.0 && self.temp.valeure_de_pente < 0.0 {
                        self.tableau[0].update_slope(self.temp.end_timestamp, self.temp.end_price, self.temp.size - 1);
                        self.temp = pente::new();
                    } else if self.tableau[0].valeure_de_pente > 0.0 && self.temp.valeure_de_pente > 0.0{
                        self.tableau[0].update_slope(self.temp.end_timestamp, self.temp.end_price, self.temp.size - 1);
                        self.temp = pente::new();
                    } else if self.temp.size > 4 {
                        self.swaptab();
                    }
                } else if self.temp.size > 4 {
                    self.swaptab();
                }
                self.affiche_structure();
                self.next_time = time;
            }
        }
    }
}