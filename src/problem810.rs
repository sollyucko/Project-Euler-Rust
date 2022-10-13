// https://projecteuler.net/problem=810
// XOR-Primes

use parking_lot::{RawRwLock, RwLock, RwLockUpgradableReadGuard, lock_api::RawRwLock as _};

pub fn xor_product(x: usize, y: usize) -> usize {
    let mut result = 0;
    let mut i: usize = 0;
    loop {
        let bit = 1 << i;
        if bit > x {
            return result;
        }
        if (bit & x) != 0 {
            result ^= y << i;
        }
        i += 1;
    }
}

struct XorPrimesCache {
    primes: Vec<usize>,
    max_checked: usize,
}

static XOR_PRIMES_CACHE: RwLock<XorPrimesCache> = RwLock::const_new(RawRwLock::INIT, XorPrimesCache { primes: Vec::new(), max_checked: 1 });

pub fn xor_primes() -> impl Iterator<Item = usize> {
    _xor_primes(None)
}

fn _xor_primes(limit: Option<usize>) -> impl Iterator<Item = usize> {
    XorPrimes {
        idx: 0,
        limit,
    }
}

struct XorPrimes {
    idx: usize,
    limit: Option<usize>,
}

impl Iterator for XorPrimes {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let cache_r = XOR_PRIMES_CACHE.upgradable_read();
        if let Some(&prime) = cache_r.primes.get(self.idx) {
            self.idx += 1;
            return Some(prime);
        }
        let mut cache = RwLockUpgradableReadGuard::upgrade(cache_r);
        loop {
            cache.max_checked += 1;
            if let Some(limit) = self.limit {
                if cache.max_checked >= limit {
                    return None;
                }
            }
            let x = cache.max_checked;
            let result = is_xor_prime(x);
            if result {
                cache.primes.push(x);
                self.idx += 1;
                return Some(x);
            }
        }
    }
}

pub fn is_xor_prime(x: usize) -> bool {
    let cache_r = XOR_PRIMES_CACHE.read();
    if x <= cache_r.max_checked {
        return cache_r.primes.binary_search(&x).is_ok();
    }
    drop(cache_r);
    for p in _xor_primes(Some(x)) {
        for i in p..x {
            if xor_product(p, i) == x {
                return false;
            }
        }
    }
    true
}

pub fn main() {
    println!("{}", xor_product(7, 3));
    println!();
    for x in xor_primes().take(10) {
        println!("{}", x);
    }
    println!();
    println!("{}", xor_primes().nth(10).unwrap());
    println!();
    println!("{}", xor_primes().nth(5_000_000).unwrap());
}
