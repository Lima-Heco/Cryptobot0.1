
pub mod tendances {
	use std::clone::Clone;
	pub struct pente {
		pub initialized: bool,
		pub size: i64,
		pub start_price: f64,
		pub start_timestamp: i64,
		pub end_price: f64,
		pub end_timestamp: i64,
		pub valeure_de_pente: f64,
    }
	impl pente {
		
		pub fn calculate_slope(x1: i64, y1: f64, x2: i64, y2: f64) -> f64 {
			if x1 == x2 {
				0.0
			} else {
				(y2 - y1) as f64/ (x2 - x1) as f64
			}
		}

		pub fn new() -> Self {
				pente{
					initialized: false,
					size: 0,
					start_price: 0.0,
					start_timestamp: 0,
					end_price: 0.0,
					end_timestamp: 0,
					valeure_de_pente: 0.0,
				}
		}

		pub fn newandinit(x1: i64, y1: f64, x2: i64, y2: f64) -> Self {
			if x1 == x2 {
				pente{
					initialized: true,
					size: 2,
					start_price: y1,
					start_timestamp: x1,
					end_price: y2,
					end_timestamp: x2,
					valeure_de_pente: 0.0,
				}
			} else {
				let pentev: f64 = (y2 - y1) as f64 / (x2 - x1) as f64;
				pente{
					initialized: true,
					size: 2,
					start_price: y1,
					start_timestamp: x1,
					end_price: y2,
					end_timestamp: x2,
					valeure_de_pente: pentev,
				}
			}
		}

		pub fn init(self: &mut Self, x1: i64, y1: f64, x2: i64, y2: f64, s: i64) {
			self.initialized = true;
			self.size = s;
			self.start_price = y1;
			self.start_timestamp = x1;
			self.end_price = y2;
			self.end_timestamp = x2;
			self.valeure_de_pente = Self::calculate_slope(0, y1, s, y2);
		}

		pub fn update_slope(self: &mut Self, x2: i64, y2: f64, s: i64)
		{
			self.end_price = y2;
			self.end_timestamp = x2;
			self.size += s;
			self.valeure_de_pente = Self::calculate_slope(0, self.start_price, self.size, y2);
		}

	}
	impl Clone for pente {
		fn clone(&self) -> Self {
			// Créez une nouvelle instance de `pente` avec les mêmes valeurs que la structure actuelle
			pente {
				initialized: self.initialized,
				size: self.size,
				start_price: self.start_price,
				start_timestamp: self.start_timestamp,
				end_price: self.end_price,
				end_timestamp: self.end_timestamp,
				valeure_de_pente: self.valeure_de_pente,
			}
		}
	}


}