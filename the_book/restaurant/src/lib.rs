pub mod front_of_house;

pub fn foh(){
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    
    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();
}
