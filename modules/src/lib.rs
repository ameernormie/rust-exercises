mod front_of_house; // Looks for the file in src/front_of_house

use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist()
}
