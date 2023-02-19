fn main() {
    println!("Hello, world!");
    for n in 0..=50 {
        if let Some(v) = fibo(n) {
            println!("fibo({n}) = {}", v);
        } else {
            break;
        }
        /*match fibo(n) {
            None => break,
            Some(fibo_n) => println!("fibo({n}) = {}", fibo_n),
        }*/
    }
}


fn fibo(n: u32) -> Option<u32> {
    if n < 2 {
        return Some(n);
    }
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 2..=n {
        let temp = b;
        match a.checked_add(b) {
            Some(sum) => b = sum,
            _ => return None,
        }
        a = temp;
    }
    Some(b)
}
