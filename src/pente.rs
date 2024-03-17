pub mod tendances {
	pub struct pente {
		initialized: bool,
		croissant: bool,
		size: i32,
		start_price: f64,
		start_timestamp: i64,
		end_price: f64,
		start_timestamp: i64,
		valeure_de_pente: f64,
    }
	impl pente {

			//________________________________NEW_________________________________//
		pub fn new(x1: i64, y1: f64, x2: i64, y2: f64) -> Self {
			if x1 == x2 {
				pente{
					initialized: true,
					croissant: true,
					size: 2,
					start_price: y1,
					start_timestamp: x1,
					end_price: y2,
					start_timestamp: x2,
					valeure_de_pente: 0.0,
				}
			} else {
				let pentev = (y2 - y1) / (x2 - x1);
				if pentev < 0 {
					let pentedir = false;
				} else {
					let pentedir = true;
				}
				pente{
					initialized: true,
					croissant: pentedir,
					size: 2,
					start_price: y1,
					start_timestamp: x1,
					end_price: y2,
					start_timestamp: x2,
					valeure_de_pente: pentev,
				}
				
			}
		}
	}

	pub fn calculate_slope(x1: i64, y1: f64, x2: i64, y2: f64) -> f64 {
		if x1 == x2 {
			0.0
		} else {
			(y2 - y1) / (x2 - x1)
		}
	}
}