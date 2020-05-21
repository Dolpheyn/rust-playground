use std::fs::File;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let s = read_username_from_file()?;

    // Handle Result<T, E> using match
    // if main does not return Result.
    // match s {
    //     Ok(s) => println!("{}", s),
    //     Err(e) => println!("{:?}", e),
    // }
    
    println!("{}", s);
    Ok(())
}

// Handle result using '?' operator. Must explicitly tell that
// the function returns Result<ok_type, error_type>
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new(); 
    f.read_to_string(&mut s)?;
    Ok(s)
}
