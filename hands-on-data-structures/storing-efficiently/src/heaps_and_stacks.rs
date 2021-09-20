use std::mem::size_of;

struct MyStruct {
    a: u8,
    b: u8,
    c: u8,
}

fn main() {
    // A struct containing 3 u8 is the same size of 3 u8,
    // which means that there are no size overhead.
    assert_eq!(size_of::<MyStruct>(), 3 * size_of::<u8>());
    assert_eq!(size_of::<[MyStruct; 2]>(), 3 * size_of::<u8>() * 2);

    // The String type is a Vec of u8 `Vec<u8>`
    // **There's also an allocator and PhantomData but
    // they're not important for sizes here.**
    // Vec<T> {
    //   buf: RawVec<T> {
    //     ptr: ptr::Unique<T> {
    //       pointer: *const T
    //     },
    //     cap: usize
    //   },
    //   len: usize
    // }
    // So, String = Vec<u8>
    // = pointer to u8 + 2 usize(len and capacity)
    //
    // Which is true for all T, because a pointer to T (*const T)
    // is always 8. usize for a 64 bit computer is 8, 32 bit
    // is 4.
    assert_eq!(size_of::<Vec<u8>>(), size_of::<String>());
    assert_eq!(
        size_of::<String>(),
        size_of::<*const u8>() + 2 * size_of::<usize>()
    );
    assert_eq!(size_of::<Vec<String>>(), size_of::<String>());

    let v: Vec<u8> = vec![1, 2, 3];

    println!("{:?}", &v[0] as *const _);
    println!("{:?}", &v[1] as *const _);
}
