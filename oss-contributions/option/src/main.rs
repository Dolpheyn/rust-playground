fn main() {
    // Example for From<&'a mut Option<T>> for Option<&'a mut T> implementation
    // Converts from &mut Option<T> to Option<&mut T>
    let mut s = Some(String::from("Hello")); 
    let o: Option<&mut String> = Option::from(&mut s);
    match o {
	Some(t) => *t = String::from("Hello, Rustaceans!"),
	None => (),
    }
    assert_eq!(Some(String::from("Hello, Rustaceans!")), s);
	
    // Example for From<&'a Option<T>> for Option<&'a T> implementation
    // Converts from &Option<t> to Option<&T>
    let s: Option<String> = Some(String::from("Hello, Rustaceans!")); 
    let o: Option<usize> = Option::from(&s).map(|ss: &String| ss.len());
    assert_eq!(Some(18), o);
    println!("Can still print s: {:?}",  s);

    // Example for From<T> for Option<T>
    // Converts T to Option::Some(T)
    let o: Option<u8> = Option::from(67);
    assert_eq!(o.is_some(), true);
    assert_eq!(Some(67), o);
}
