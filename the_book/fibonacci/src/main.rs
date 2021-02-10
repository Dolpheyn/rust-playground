use std::io;


fn main() {
    let mut number: String = String::new();
    
    println!("Fibonacci");
    println!("Please enter a number");
    io::stdin()
	.read_line(&mut number)
	.expect("Failed to read line!");

    let number: u32 = number.trim().parse()
	.expect("Please enter a number!");

    println!("You entered {}", number);
    let f = fibonacci(number);
    println!("Fibonacci of {}: {}", number, f);
}

fn fibonacci(n: u32) -> u32 {
    match n {
	0 => 0,
	1 => 1,
	_ => fibonacci(n-1) + fibonacci(n-2),
    }
}
