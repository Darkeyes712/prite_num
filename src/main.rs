fn main() {
    println!("{:?}", factors(12));
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    for i in 1..n {
        if n % i == 0 && i != 1 {
            if n / i % i == 0 {
                factors.push(i);
            }
            // println!("{}", devider);
            // factors.push(i);
        }
    }

    factors
}
