pub mod analyseBFC_1 {

    pub struct marqueures {
        pub findv1_try: i64,
        pub findv1_succes: i64,
        pub findv2_try: i64,
        pub findv2_succes: i64,
        pub findivp1_try: i64,
        pub findivp1_succes: i64,
        pub findp1_try: i64,
        pub findp1_succes: i64,
    }
    impl marqueures {
        pub fn new() -> Self {
            marqueures {
                findv1_try: 0,
                findv1_succes: 0,
                findv2_try: 0,
                findv2_succes: 0,
                findivp1_try: 0,
                findivp1_succes: 0,
                findp1_try: 0,
                findp1_succes: 0,
            }
        }
        pub fn print(self: &mut Self) {
            println!("  comptrendu des investissements : \n\n");
            println!("      findv1:     tentatives: {} reussites: {}\n", self.findv1_try, self.findv1_succes);
            if self.findv1_try > 0 && self.findv1_succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.findv1_succes * 100) / self.findv1_try);
            }
            println!("      findv2:     tentatives: {} reussites: {}\n", self.findv2_try, self.findv2_succes);
            if self.findv2_try > 0 && self.findv2_succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.findv2_succes * 100) / self.findv2_try);
            }
            println!("      findivp1:     tentatives: {} reussites: {}\n", self.findivp1_try, self.findivp1_succes);
            if self.findivp1_try > 0 && self.findivp1_succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.findivp1_succes * 100) / self.findivp1_try);
            }
            println!("      findp1:     tentatives: {} reussites: {}\n", self.findp1_try, self.findp1_succes);
            if self.findp1_try > 0 && self.findp1_succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.findp1_succes * 100) / self.findp1_try);
     }
           }
    }
}