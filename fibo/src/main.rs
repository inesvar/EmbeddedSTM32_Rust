fn main() {
    println!("Hello, world!");
    for n in 0..=42 {
        println!("fibo({n}) = {}", fibo(n));
    }
}


fn fibo(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = b;
        b = a + b;
        a = temp;
    }
    b
}
