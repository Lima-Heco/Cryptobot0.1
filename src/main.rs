mod get_btc;
mod fenetre;

use std::io;
use crate::fenetre::ihm;
use crate::get_btc::get_bitcoin;
use std::sync::{Arc, Mutex, RwLock};
use std::{thread, time};

fn main() {
    let mut ordre = String::new();
    let mut btc_price = get_bitcoin::btcprice::new();
    let should_stop = Arc::new(Mutex::new(false));
    let should_stop_clone = Arc::clone(&should_stop);



    println!("Cryptobot : ");
    io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
    while ordre != "exit\n" {
        if ordre == "data -s\n" {
            let should_stop_clone = Arc::clone(&should_stop);
            let btc_price = Arc::clone(&btc_price);
            let handle = thread::spawn(move || {
                get_bitcoin::get_btc_in_data(btc_price, should_stop_clone);
            });
        }
        if ordre == "data -d\n" {
            *should_stop.lock().unwrap() = true;
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
