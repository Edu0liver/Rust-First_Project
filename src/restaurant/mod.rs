mod pizzaOrder {
    
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub topping: String
    }

    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza {
            return Pizza {
                dough: String::from("Regular dough"),
                cheese: String::from("Mozzarella"),
                topping: String::from(topping)
            }
        }
    }

    pub mod helpCustomer {

        fn seatAtTable() {
            println!("Customer seated at table");
        }
        
        fn serveCustomer(custPizza: super::Pizza) {
            println!("The customer is served a regular pizza with {}", custPizza.topping);
        }

        pub fn takeOrder() {
            seatAtTable();

            let custPizza: super::Pizza = super::Pizza::lunch("Veggies");

            serveCustomer(custPizza);
        }

    }

}

pub fn orderFood() {
    crate::restaurant::pizzaOrder::helpCustomer::takeOrder();
}
