//mod Data;
mod Data {pub mod get_btc;}
mod Screen {pub mod fenetre;}
mod BFC_1 {pub mod Bot_read{ pub mod pente;}}

use std::io;
use Screen::fenetre::ihm;
use Data::get_btc::get_bitcoin; 
use std::sync::{Arc, Mutex, RwLock};
use std::{thread, time};

fn main() {
    let mut ordre = String::new();
    let btc_price = get_bitcoin::btcprice::new();
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
