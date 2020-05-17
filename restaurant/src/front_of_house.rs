pub mod hosting {
    pub fn add_to_waitlist() {println!("front_of_house::hosting::add_to_waitlist()");}

    pub fn seat_at_table() {println!("front_of_house::hosting::seat_at_table()");}
}

pub mod serving {
    pub fn take_order() {println!("front_of_house::serving::take_order()");}	

    pub fn serve_order() {println!("front_of_house::serving::serve_order()");}

    pub fn take_payment() {println!("front_of_house::serving::take_payment()");}	
}
