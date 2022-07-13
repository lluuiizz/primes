use std::env;
use std::process;

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {
        println!("Missing range limit argument!!!");

        process::exit(0);
   }


   let n_range = args[1].to_string().parse::<i32>().unwrap();
   let primes = primes_up_to_n(n_range, find_all_tangible_divisors(n_range));
   for num in primes.iter() {
        println!("{}", num)
   }
}


fn find_all_tangible_divisors(n: i32) -> Vec<i32> {
   let mut divisors: Vec<i32> = Vec::new(); 
   divisors.push(2); 
   for i in 3..n {
       if i % 2 != 0 {
            divisors.push(i);
       } 
   }

   return divisors;
}

fn primes_up_to_n (n: i32, tangible_divisors: Vec<i32>) -> Vec<i32> {
    let mut primes: Vec<i32> = Vec::new();
    if n >= 2 {
        primes.push(2)
    }
    for num in 3..n+1 {
        let mut num_is_prime = true;
        for divisor in tangible_divisors.iter() {
            if *divisor > num {
                break;
            }
            if num % divisor == 0 && divisor + 1 < num {
               num_is_prime = false;  
               break;
            }
        }
        if num_is_prime {primes.push(num);}
    }  
    return primes;
}
