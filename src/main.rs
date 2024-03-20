//---------------------------Mod and use--------------------//
    mod Data {pub mod get_btc;}
    mod Screen {pub mod fenetre;}
    mod BFC_1 {pub mod brain; pub mod Bot_read{ pub mod pente; pub mod pente_view;}}

    use Screen::fenetre::ihm;
    use Data::get_btc::get_bitcoin; 
    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};
    use BFC_1::brain::BFC_1_brain;
//---------------------------main---------------------------//

    fn main() {
        let mut ordre = String::new();
        let btc_price = get_bitcoin::btcprice::new();
        let should_stop = Arc::new(Mutex::new(false));
        let should_stop_data = Arc::clone(&should_stop);
        let should_stop2 = Arc::new(Mutex::new(false));
        let should_stop_bot = Arc::clone(&should_stop2);



        println!("Cryptobot : ");
        io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
        while ordre != "exit\n" {
            if ordre == "data -s\n" {
                let should_stop_data = Arc::clone(&should_stop);
                let btc_price = Arc::clone(&btc_price);
                let handle = thread::spawn(move || {
                    get_bitcoin::get_btc_in_data(btc_price, should_stop_data);
                });
            }
            if ordre == "data -d\n" {
                *should_stop.lock().unwrap() = true;
                thread::sleep(time::Duration::from_secs(2));
                *should_stop.lock().unwrap() = false;
            }
            if ordre == "bot -s\n" {
                let should_stop_bot = Arc::clone(&should_stop2);
                let btc_price = Arc::clone(&btc_price);
                let handle = thread::spawn(move || {
                    BFC_1_brain::bfcbrain(btc_price, should_stop_bot);
                });
            }
            if ordre == "bot -d\n" {
                *should_stop2.lock().unwrap() = true;
                thread::sleep(time::Duration::from_secs(2));
                *should_stop2.lock().unwrap() = false;
            }

            /*if ordre == "f\n" {
                ihm::fenetre()
            }*/
            ordre.clear();
            println!("Cryptobot : ");
            io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
        }
        *should_stop.lock().unwrap() = true;
    }
