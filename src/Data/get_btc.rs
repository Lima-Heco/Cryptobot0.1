pub mod get_bitcoin {
    use std::process::Command;
    use reqwest::blocking;
    use rusqlite::{params, Connection};
    use serde_json;
    use std::{thread, time};
    use std::sync::{Arc, Mutex, RwLock};

//-----------------------------STRUCT-------------------------------//

    pub struct btcprice {
        pub v_btctousd: Vec<f64>,
        pub v_btctousdless: Vec<f64>,
        pub v_btctousdmore: Vec<f64>,
        pub v_timestamp: Vec<i64>,
        pub v_differences: Vec<i64>,
        pub btctousd: f64,
        pub startbtctousd: f64,
        pub btctousdless: f64,
        pub btctousdmore: f64,
        pub differences: f64,
        pub timestamp: i64,
        pub next_time:bool,
    }
    impl btcprice {
    
        pub fn new() -> Arc<RwLock<Self>> {
            Arc::new(RwLock::new(btcprice {
            v_btctousd: Vec::new(),
            v_btctousdless: Vec::new(),
            v_btctousdmore: Vec::new(),
            v_timestamp: Vec::new(),
            v_differences: Vec::new(),
            btctousd: 0.0,
            startbtctousd: 0.0,
            btctousdless: 0.0,
            btctousdmore: 0.0,
            differences: 0.0,
            timestamp: 0,
            next_time: true,
            }))
        }

        pub fn get_new(self: &mut Self, n1: f64, n2: f64, n3: f64, n4: i64) {
            self.v_btctousd.push(n1);
            self.v_btctousdmore.push(n2);
            self.v_btctousdless.push(n3);
            self.v_timestamp.push(n4);
        }
    }
//-----------------------------FN_GET_DATA--------------------------//

    fn create_database_and_store_data(btc_price: Arc<RwLock<btcprice>>) -> Result<(), Box<dyn std::error::Error>> {
        // Connexion à la base de données SQLite
        //let conn = Connection::open("bitcoin_database.db")?;
        let timestamp = chrono::Utc::now().timestamp();
        
        // Créer la table si elle n'existe pas déjà
        /*conn.execute(
            "CREATE TABLE IF NOT EXISTS bitcoin_data (
                priceUsd REAL NOT NULL,
                MAXpriceUsd REAL NOT NULL,
                MINpriceUsd REAL NOT NULL,
                timestamp INTEGER NOT NULL
            )",
            params![],
        )?;*/
        if true {
            let btc_price_clone = Arc::clone(&btc_price);
            let mut price_guard = btc_price_clone.write().unwrap();
            price_guard.timestamp = timestamp;
            if price_guard.next_time == true {
                price_guard.next_time = false;            
            } else {
                price_guard.next_time = true;
            }
        }
        /*let btc_price_clone = Arc::clone(&btc_price);
        let price = {
            let price_guard = btc_price_clone.read().unwrap();
            price_guard.btctousd
        };
        let btc_price_clone = Arc::clone(&btc_price);
        let maxprice = {
            let price_guard = btc_price_clone.read().unwrap();
            price_guard.btctousdmore
        };
        let btc_price_clone = Arc::clone(&btc_price);
        let minprice = {
            let price_guard = btc_price_clone.read().unwrap();
            price_guard.btctousdless
        };*/
        /*if true {
            let mut btc = btc_price_clone.write().unwrap();
            btc.get_new(price, maxprice, minprice, timestamp);
        }*/
        /*conn.execute(
            "INSERT INTO bitcoin_data (priceUsd, MAXpriceUsd, MINpriceUsd, timestamp) VALUES (?, ?, ?, ?)",
            params![price, maxprice, minprice, timestamp],
        )?;*/

        //println!("bilan : {}\n    Max: \x1b[32m{}\x1b[0m\n    Courrant: {}\n    Min: \x1b[31m{}\x1b[0m", timestamp, maxprice, price, minprice);
        Ok(())
    }

//-----------------------------FV_FETCHBTC--------------------------//
    
    fn fetch_btcusdt() -> Result<serde_json::Value, reqwest::Error> {
        let response = reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")?;
        let data: serde_json::Value = response.json()?;
        Ok(data)
    }
//-----------------------------FN_GET_BTC---------------------------//

    pub fn get_btc_in_data(btc_price: Arc<RwLock<btcprice>>, should_stop_clone: Arc<Mutex<bool>>) {
        let mut i: i32;
        let mut min: f64;
        let mut max: f64;
        i = 1;
        min = -1.0;
        max = -1.0;

        println!("Get_data started...");
        loop {
            let btc_price_clone = Arc::clone(&btc_price);
            match fetch_btcusdt() {
                Ok(data) => {
                    let price_str = data["price"].as_str().expect("Price missing");
                    let price = price_str.parse::<f64>().expect("Failed to parse price as f64");
                    //println!("Bitcoin data: {}\n\n", price);
                    
                    
                    if true {
                        let btc_price_clone = Arc::clone(&btc_price);
                        let mut btc = btc_price_clone.write().unwrap();
                        btc.btctousd = price;

                        if i == 1 {
                            btc.startbtctousd = btc.btctousd;
                        }
                        
                        if btc.v_btctousd.len() > 0 {
                            
                        }

                        if min == -1.0 || min > price {
                            min = price;
                            btc.btctousdless = min;
                        }   
                        if max == -1.0 || max < price {
                            max = price;
                            btc.btctousdmore = max;
                        }                     
                    }
                    //println!("s: {}", i);
                    //println!("bilan :\n    Max: \x1b[32m{}\x1b[0m\n    Courrant: {}\n    Min: \x1b[31m{}\x1b[0m", max, price, min);
                    i += 1;
                    if i >= 20 {
                        let btc_price_clone = Arc::clone(&btc_price);
                        let _ = create_database_and_store_data(btc_price_clone);
                        i = 1;
                        min = -1.0;
                        max = -1.0;
                    }
                    if *should_stop_clone.lock().unwrap() {
                        break; // Sortir de la boucle si le drapeau est activé
                    }
                }
                Err(err) => eprintln!("Error fetching Bitcoin data: {:?}", err),
            }
            thread::sleep(time::Duration::from_secs(1));
        }
        println!("Get_data delleted...");
    }
}
