pub fn prime_sieve(n:u64) -> Vec<u64> {
    let mut primes=(3..n).step_by(2).collect::<Vec<_>>();
    let mut result=vec![2];
    while primes.len()>2 {
        let prime=(primes[0],primes[1],primes[2]);
        milans_rust_logging::warn!("{} {} {}",prime.0,prime.1,prime.2);
        if prime.0>n/prime.0 {
            break;
        }
        result.push(prime.0);
        result.push(primes[1]);
        result.push(primes[2]);
        primes=primes.iter().filter(|&&x| x%prime.0!=0&&x%prime.1!=0&&x%prime.2!=0).cloned().collect();
    }
    result.append(&mut primes);
    result
}
