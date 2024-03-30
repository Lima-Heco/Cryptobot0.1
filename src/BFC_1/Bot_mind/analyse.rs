pub mod analyseBFC_1 {

    pub struct marqueures {
        pub name: String,
        pub ttry: i64,
        pub succes: i64,
        pub propice: i32,
    }
    impl marqueures {
        pub fn new(name: String) -> Self {
            marqueures {
                name: name,
                ttry: 0,
                succes: 0,
                propice: 0,
            }
        }
        pub fn print(self: &mut Self) {
            println!("  comptrendu des investissements : \n\n");
            println!("      {}:     tentatives: {} reussites: {}\n", self.name, self.ttry, self.succes);
            if self.ttry > 0 && self.succes > 0{
                println!("           pourcentage de reussites: {}%\n\n", (self.succes * 100) / self.ttry);
            } else if self.ttry > 0 {
                println!("           pourcentage de reussites: 0%\n\n");
            }
        }
    }
}