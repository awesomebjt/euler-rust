fn main() {
    println!("ProjectEuler.net Problem 3:\nHighest prime factor of 600851475143.\n");
    let mut n = 600851475143;
    //First, we need to find all the prime numbers that might be factors of our number.
    //This should save time over just brute force
    //Create a Seive of Eratosthenes. An array of bools going from 0 to 600851475143.sqrt()
    let max_factor = (n as f64).sqrt().round() as usize;
    let mut prime_flags : Vec<bool> = vec![true;max_factor];
    let mut primes : Vec<u64> = vec![];
    let mut result : Vec<u64> = vec![];
    let mut i = 2;
    prime_flags[0] = false;
    prime_flags[1] = false;
    while i<max_factor {
        if prime_flags[i] == true {
            let mut j = i;
            while i*j<max_factor {
                //If you find a prime number, set all its multiples to false (not prime)
                prime_flags[i*j] = false;
                j = j + 1;
            }
        }
        i = i + 1;
    }
    //Now that the sieve has left all the prime indexes of prime_flags[] as "true",
    //We fill a resizable array of integers and fill it with the prime numbers.
    //We might be able to do without this but it will make later steps easier to understand
    i = 2;
    while i<max_factor {
        if prime_flags[i] == true {
            primes.push(i as u64);
        }
        i = i + 1;
    }
    //primes is now a Vec filled with prime integers that are <= the square root of the big number.
    //Finally, we do what looks a bit like the brute-force algorithm except we only test primes.
    i = 0;
    while i<primes.len() && n > 1 {
        while n % primes[i] == 0 {
            n = n / primes[i];
            result.push(primes[i]);
        }
        i = i + 1;
    }
    //Print the Answer
    println!("Answer: {:?}", result);
}
