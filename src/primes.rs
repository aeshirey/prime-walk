use reikna::prime;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, LineWriter, Lines};

fn generate_primes(limit: i64, filename: &str) {
    let results = prime::prime_sieve(limit as u64);

    let fh = File::create(filename).unwrap();

    let mut writer = LineWriter::new(fh);

    for prime in results {
        write!(writer, "{}\n", prime).unwrap();
    }
}

pub struct PrimeIter {
    bufread: Lines<BufReader<File>>,
}

impl PrimeIter {
    pub fn new(num_primes: i64) -> Self {
        let filename = format!("primes_{}.txt", num_primes);
        if !std::path::Path::new(&filename).exists() {
            println!("Primes file doesn't exist. Creating it now...");
            generate_primes(num_primes, &filename);
        }

        let fh = File::open(filename).unwrap();
        let bufread = BufReader::new(fh).lines();

        PrimeIter { bufread }
    }
}

impl Iterator for PrimeIter {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.bufread.next().map(|s| s.unwrap().parse().unwrap())
    }
}
