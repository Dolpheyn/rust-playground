pub mod public_nested;
mod private_nested;
pub mod resurface_deeply_nested;

pub fn function() {
    println!("called `module::function()`");
}

pub fn indirect_private_nested_function() {
    print!("called `module::indirect_private_nested_function()`");
    print!(" that\n> ");
    private_nested::function();
}
