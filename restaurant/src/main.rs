use restaurant::front_of_house::{hosting, serving};

fn main() {
    hosting::add_to_waitlist();
    serving::take_order();
}
