use crate::module::public_nested::deeply_nested;

pub fn function() {
    print!("called `module::resurface_deeply_nested::function()`");
    print!(" that\n> ");
    deeply_nested::function();
}
