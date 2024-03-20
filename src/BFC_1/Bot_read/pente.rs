
pub mod tendances {
	pub struct pente {
		initialized: bool,
		size: i32,
		start_price: f64,
		start_timestamp: i64,
		end_price: f64,
		end_timestamp: i64,
		valeure_de_pente: f64,
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

		pub fn init(self: &mut Self, x1: i64, y1: f64, x2: i64, y2: f64, s: i32) {
			self.initialized = true;
			self.size = s;
			self.start_price = y1;
			self.start_timestamp = x1;
			self.end_price = y2;
			self.end_timestamp = x2;
			self.valeure_de_pente = Self::calculate_slope(x1, y1, x2, y2);
		}

		pub fn update_slope(self: &mut Self, x2: i64, y2: f64, s: i32)
		{
			self.end_price = y2;
			self.end_timestamp = x2;
			self.size = s;
			self.valeure_de_pente = Self::calculate_slope(self.start_timestamp, self.start_price, x2, y2);
		}

	}


}