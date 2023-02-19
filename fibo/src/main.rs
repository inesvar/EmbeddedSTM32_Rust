fn main() {
    println!("Hello, world!");
    for n in 0..=50 {
        println!("fibo({n}) = {}", fibo(n));
    }
}


fn fibo(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 2..=n {
        let temp = b;
        b = a.checked_add(b).unwrap();
        a = temp;
    }
    b
}
