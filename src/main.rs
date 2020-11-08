use clap::{Arg, App};
use std::io::Write;

struct PrimeTable {
    n: usize,
    primes: Vec<u32>,
}

impl PrimeTable {
    pub fn new(n: usize) -> PrimeTable {
        let primes = vec![0xFFFFFFFF; n >> 3];
        let mut res = PrimeTable { n, primes, };
        res.set_trivial_non_primes();
        res.calculate_primes();
        res
    }

    fn set_trivial_non_primes(&mut self) {
        self.turn_off(0);
        self.turn_off(1);
    }

    fn calculate_primes(&mut self) {
        let n = self.n as u64;
        let m = (self.n as f64).sqrt() as u64 + 1;
        for p in 2..=m {
            if !self.is_prime(p as usize) {
                continue;
            }

            let mut i = 2 * p;
            loop {
                if i > n {
                    break
                }
                self.turn_off(i as usize);
                i += p;
            }
        }
    }

    fn turn_off(&mut self, n: usize) {
        self.primes[n >> 5] &= !(0x1 << (n & 31));
    }

    fn turn_on(&mut self, n: usize) {
        self.primes[n >> 5] |= 0x1 << (n & 31);
    }

    pub fn is_prime(&self, n: usize) -> bool {
        ((self.primes[n >> 5] >> (n & 31)) & 0x01) != 0
    }

    pub fn print_table(&self, rows: usize, cols: usize) {
        for i in 0..rows {
            for j in 0..cols {
                if self.is_prime(cols*i + j) {
                    print!("■");
                } else {
                    print!("□");
                }
            }
            println!("");
        }
    }
}

fn main() {
    let matches = App::new("")
        .version("0.0.1")
        .author("rugamaga")
        .about("generate primes")
        .arg(
            Arg::with_name("max")
                .short("m")
                .long("max")
                .help("Maximum checking number")
                .takes_value(true))
        .get_matches();

    let m: usize = matches
        .value_of("max")
        .unwrap_or("")
        .parse()
        .unwrap_or(std::usize::MAX);

    println!("max candidates: {}", m);
    let primes = PrimeTable::new(m);
    println!("Table created.");

    primes.print_table(10, 10);

    loop {
        print!("input N: ");
        std::io::stdout().flush().unwrap();
        let mut s = String::new();
        let n: usize = std::io::stdin()
            .read_line(&mut s)
            .ok()
            .and_then(|_| s.trim().parse().ok())
            .unwrap_or(0);
        if n > (m as usize) {
            println!("{} is over than max candidate {}", n, m);
            continue;
        }
        println!("{} is prime? : {}", n, primes.is_prime(n));
    }
}
