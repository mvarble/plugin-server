fn solver(factors: &[u64], upper_bound: u64) -> u64 {
    let mut sum = 0;
    for multiple in 1..upper_bound {
        for factor in factors {
            if multiple % factor == 0 {
                sum = sum + multiple;
                break;
            }
        }
    }
    sum
}

#[no_mangle]
pub unsafe extern "C" fn solve(factor_count: u64, factors: *const u64, upper_bound: u64) -> u64 {
    let factors = std::slice::from_raw_parts(factors, factor_count as usize);
    solver(factors, upper_bound)
}
