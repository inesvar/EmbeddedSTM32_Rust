fn main() {
    println!("Hello, world!");
    for n in 0..=42 {
        println!("fibo({n}) = {}", fibo(n));
    }
}


fn fibo(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fibo(n-1) + fibo(n-2)
    }
}
