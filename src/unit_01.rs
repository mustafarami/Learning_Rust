pub(crate) fn fib(n: u8) -> u16 {
    if (n == 0 || n == 1) {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub(crate) fn prints() {
    println!("Hello"); // no args
    println!("Hello {}", "world"); // simple
    println!("Hello {1} {0}", "world", 1); // positional
    println!("{value}", value = 4); // named
    println!("Hello {:?}", ("world", 5)); // debug
    println!("Hello {:#?}", ("world", 5)); // pretty-print
}
