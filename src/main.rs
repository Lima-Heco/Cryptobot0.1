//---------------------------Mod and use------------------------------------------------//
    mod Data {pub mod get_btc;}
    mod Screen {pub mod fenetre;}
    mod BFC_1 {pub mod brain; pub mod Bot_read{ pub mod pente; pub mod pente_view;} pub mod Bot_mind{pub mod algo_1; pub mod analyse;}}

    use Screen::fenetre::ihm;
    use Data::get_btc::get_bitcoin; 
    use std::sync::{Arc, Mutex, RwLock};
    use std::{thread, time, io};
    use BFC_1::brain::BFC_1_brain;
    use crate::BFC_1::Bot_mind::analyse::analyseBFC_1::marqueures;

//---------------------------Solde struct-----------------------------------------------//

    pub struct solde {
        pub solde: f64,
    }

    impl solde {
        pub fn new(depart: f64) -> Self {
            solde {
                solde: depart,
            }
        }

        fn have_found(self: &mut Self, mise: f64) -> bool{
            if self.solde >= mise {
                return true;
            } else {
                println!("ERREURE: solde insufisant pour miser {}$", mise);
                return false;
            }
        }
        pub fn winorloose(self: &mut Self, mise: f64, win: bool) {
            if !self.have_found(mise) {
                return;
            }
            self.solde -= mise / 100.0;
            if win {
                self.solde += mise;
            } else {
                self.solde -= mise;
            }
        }
    }
//---------------------------main-------------------------------------------------------//

    fn main() {
        let mut ordre = String::new();
        let btc_price = get_bitcoin::btcprice::new();
        let infobot = Arc::new(Mutex::new(marqueures::new(String::from("test"))));
        let should_stop = Arc::new(Mutex::new(false));
        let should_stop_data = Arc::clone(&should_stop);
        let should_stop2 = Arc::new(Mutex::new(false));
        let should_stop_bot = Arc::clone(&should_stop2);
        let infobot2 = Arc::new(Mutex::new(marqueures::new(String::from("bot de reconaissances de chutes a duree courtes avec recherche de situation"))));
        
        let mysolde = Arc::new(Mutex::new(solde::new(200.0)));


        println!("---------------------Cryptobot : ");
        io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
        while ordre != "exit\n" {
//---------------------------recup data-------------------------------------------------//
            if ordre == "data -s\n" {
                let should_stop_data = Arc::clone(&should_stop);
                let btc_price = Arc::clone(&btc_price);
                let handle = thread::spawn(move || {
                    get_bitcoin::get_btc_in_data(btc_price, should_stop_data);
                });
                thread::sleep(time::Duration::from_secs(2));
            }
//---------------------------arret recup data-------------------------------------------//
            if ordre == "data -d\n" {
                *should_stop.lock().unwrap() = true;
                thread::sleep(time::Duration::from_secs(2));
                *should_stop.lock().unwrap() = false;
            }
//---------------------------demmarage bot----------------------------------------------//
            if ordre == "bot -s\n" {
                
                let should_stop_bot = Arc::clone(&should_stop2);
                let mysolde_clone = Arc::clone(&mysolde);
                let info = Arc::clone(&infobot);
                let btc_price = Arc::clone(&btc_price);
                let handle = thread::spawn(move || {
                    BFC_1_brain::bfcbrain(btc_price, should_stop_bot, info, 3, mysolde_clone);
                });
                thread::sleep(time::Duration::from_secs(2));
            }

            if ordre == "cbot\n" {
                let should_stop_bot = Arc::clone(&should_stop2);
                let mysolde_clone = Arc::clone(&mysolde);
                let info2 = Arc::clone(&infobot2);
                let btc_price = Arc::clone(&btc_price);
                let handle = thread::spawn(move || {
                    BFC_1_brain::bfcbrain(btc_price, should_stop_bot, info2, 3, mysolde_clone);
                });
                thread::sleep(time::Duration::from_secs(2));
            }
//---------------------------arret bot--------------------------------------------------//
            if ordre == "bot -d\n" {
                *should_stop2.lock().unwrap() = true;
                thread::sleep(time::Duration::from_secs(2));
                *should_stop2.lock().unwrap() = false;
            }
//---------------------------jouer haut-------------------------------------------------//
            if ordre == "play -t\n" {
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
                        if true {
							let mysold_clone = Arc::clone(&mysolde);
							let mut onlinesold = mysold_clone.lock().unwrap();
							onlinesold.winorloose(10.0, false);
						}
                    } else {
                        if true {
							let mysold_clone = Arc::clone(&mysolde);
							let mut onlinesold = mysold_clone.lock().unwrap();
							onlinesold.winorloose(10.0, true);
						}
                        println!("\n\n\n\n\nwin...\n\n\n\n\n\n\n");
                    }
                }
            }
//---------------------------affiche les informations d'investissement------------------//
            if ordre == "info\n" {
                let info = Arc::clone(&infobot2);
                
                let mut test = info.lock().unwrap();
                test.print();
                thread::sleep(time::Duration::from_secs(2));
            }
//---------------------------jouer bas--------------------------------------------------//

            if ordre == "play -b\n" {
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
                        if true {
							let mysold_clone = Arc::clone(&mysolde);
							let mut onlinesold = mysold_clone.lock().unwrap();
							onlinesold.winorloose(10.0, false);
						}
                    } else {
                        println!("\n\n\n\n\nwin...\n\n\n\n\n\n\n");
                        if true {
							let mysold_clone = Arc::clone(&mysolde);
							let mut onlinesold = mysold_clone.lock().unwrap();
							onlinesold.winorloose(10.0, true);
						}
                    }
                }
            }
//---------------------------IHM--------------------------------------------------------//
            if ordre == "f\n" {
                let should_stop_bot = Arc::clone(&should_stop2);
                let mysolde_clone = Arc::clone(&mysolde);
                let info2 = Arc::clone(&infobot2);
                let btc_price = Arc::clone(&btc_price);
                ihm::fenetre(btc_price, mysolde_clone, should_stop_bot, info2)
            }
            ordre.clear();
            println!("--------------------Cryptobot : ");
            io::stdin().read_line(&mut ordre).expect("Echec de la lecture de l'entree utilisateur");
        }
//=======================================================================================//
        *should_stop.lock().unwrap() = true;
        *should_stop2.lock().unwrap() = true;
        thread::sleep(time::Duration::from_secs(2));
        println!("Merci!!");
    }
