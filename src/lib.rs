use rand::Rng;

pub fn pi_mc_sample(n: &u64) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let mut count: u64 = 0;
    for _ in 0..*n {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        if x.powi(2)+y.powi(2) < 1.0 {count += 1};
    };
    let nfloat = *n as f64;
    let estimate = count as f64 / nfloat * 4.0;
    let sem = (estimate * (4.0-estimate) / (nfloat - 1.0)).sqrt();
    (estimate, sem)
}