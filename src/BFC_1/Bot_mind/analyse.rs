pub mod analyseBFC_1 {

    pub struct marqueures {
        pub findv2_try: i64,
        pub findv2_succes: i64,
        pub findivp1_try: i64,
        pub findivp1_succes: i64,
        pub findivp2_try: i64,
        pub findivp2_succes: i64,
        pub findp1_try: i64,
        pub findp1_succes: i64,
        pub propice: i32,
    }
    impl marqueures {
        pub fn new() -> Self {
            marqueures {
                findv2_try: 0,
                findv2_succes: 0,
                findivp1_try: 0,
                findivp1_succes: 0,
                findivp2_try: 0,
                findivp2_succes: 0,
                findp1_try: 0,
                findp1_succes: 0,
                propice: 0,
            }
        }
        pub fn print(self: &mut Self) {
            println!("  comptrendu des investissements : \n\n");
            println!("      findv2:     tentatives: {} reussites: {}\n", self.findv2_try, self.findv2_succes);
            if self.findv2_try > 0 && self.findv2_succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.findv2_succes * 100) / self.findv2_try);
            } else if self.findv2_try > 0 {
                println!("           pourcentage de reussites: 0%\n\n");
            }
            println!("      findivp1:     tentatives: {} reussites: {}\n", self.findivp1_try, self.findivp1_succes);
            if self.findivp1_try > 0 && self.findivp1_succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.findivp1_succes * 100) / self.findivp1_try);
            } else if self.findivp1_try > 0 {
                println!("           pourcentage de reussites: 0%\n\n");
            }
            println!("      findivp2:     tentatives: {} reussites: {}\n", self.findivp2_try, self.findivp2_succes);
            if self.findivp2_try > 0 && self.findivp2_succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.findivp2_succes * 100) / self.findivp2_try);
            } else if self.findivp2_try > 0 {
                println!("           pourcentage de reussites: 0%\n\n");
            }
            println!("      findp1:     tentatives: {} reussites: {}\n", self.findp1_try, self.findp1_succes);
            if self.findp1_try > 0 && self.findp1_succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.findp1_succes * 100) / self.findp1_try);
            } else if self.findp1_try > 0 {
                println!("           pourcentage de reussites: 0%\n\n");
            }
        }
    }
}