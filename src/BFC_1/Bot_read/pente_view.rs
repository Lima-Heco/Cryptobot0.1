pub mod tendance_view {
    use crate::BFC_1::Bot_read::pente::tendances::pente;

    pub struct BFC_1_view {
        tableau: [pente; 3],
    }
    
    impl BFC_1_view {
        fn new() -> Self {
            BFC_1_view {
                tableau: [
                    pente::new(),
                    pente::new(),
                    pente::new(),
                ],
            }
        }
    }
}