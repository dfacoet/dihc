use std::env;

use dihc::pi_mc_sample;

fn main () {
    let args: Vec<String> = env::args().collect();
    
    let n: u64 = (&args[1]).parse().expect("n must be a positive integer");
    // TODO: check n>0
    let (pi, err) = pi_mc_sample(&n);

    println!("MC estimate of pi with {} samples: {}±{}", n, pi, err);
}