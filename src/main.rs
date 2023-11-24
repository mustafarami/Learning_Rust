mod unit_01;
fn main() {
    unit_01::prints();
    let n = 10;
    let res = unit_01::fib(n);
    println!("fib({}) = {}", n, res);
}
