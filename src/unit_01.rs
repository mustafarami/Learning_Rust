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

/* Parameters
Identifier |     |      Return Type
     |     |     |          |
    --- ------  ------     ---
 fn mul(x: i32, y: i32) -> i32 {
 --     -  ---          --     -
 |      |   |            |     |
Keyword |  Type         return |
        |                      |
    Identifier            Begin Function
                              Body
 */
pub(crate) fn mul(x: i32, y: i32) -> i32 {
    x * y
}

//Recursion
//In Rust you can call functions recursively, 
//just like in this function for computing the greatest common divisor using Euclid’s algorithm:
pub(crate) fn gcd(m: i32, n: i32) -> i32 {
    if m == 0 {
       n.abs()
    } else {
       gcd(n % m, m)
    }
 }
 