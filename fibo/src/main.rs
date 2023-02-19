use clap::Parser;

/// Compute Fibonacci suite values
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    /// The maximal number to print the fibo value of
    value: u32,
    /// Print intermediate values
    #[arg(short, long)]
    verbose: bool,
    /// The minimum number to compute
    #[arg(short, long)]
    number: Option<u32>,
}

fn main() {
    let args = Args::parse();
    if let Some(v) = fibo(args.value) {
        println!("fibo({}) = {}", args.value, v);
    }
}


fn fibo(n: u32) -> Option<u32> {
    let args = Args::parse();
    let min :u32 = args.number.unwrap_or(0);
    if n < 2 {
        return Some(n);
    }
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for i in 2..=n {
        let temp = b;
        match a.checked_add(b) {
            Some(sum) => b = sum,
            _ => return None,
        }
        a = temp;
        if args.verbose && i >= min && i < args.value { 
            println!("fibo({i}) = {}", b);
        }
    }
    Some(b)
}
