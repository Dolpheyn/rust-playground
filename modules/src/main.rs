mod module;

fn function() {
    println!("called `function()`");
}

/// The `module::indirect_private_nested_function()`
/// can call `module::private_nested::function()`
/// because the function itself is public, so it is
/// accessible within `module`
fn main() {
    function();
    module::function();
    module::public_nested::function();
    module::indirect_private_nested_function();
    module::public_nested::deeply_nested::function();

    // Line below wont run because in src/module/mod.rs,
    // the module private_nested is not declared public
    //module::private_nested::function();
    module::resurface_deeply_nested::function();

    // In lib.rs, the project `modules` is turned into a 
    // library, and we 
    // pub use module::public_nested::deeply_nested::function as
    // resurfaced_deeply_nested_function this way, we can make 
    // items that are deep inside modules and nested modules
    // appear as our library's surface item.
    // Eg if we have a project named 'service', we can
    // pub use crate::module_with_runner::runner::run() in lib.rs
    // and use service::run in main.rs without including the
    // module_with_runner module and running 
    // module_with_runner::runner::run() function.
    modules::resurfaced_deeply_nested_function();
}
