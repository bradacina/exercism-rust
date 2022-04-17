use std::collections::HashSet;

fn multiples(limit: u32, factor: u32) -> Vec<u32> {
    if factor == 0 {
        return vec![0];
    }

    let q: u32 = (limit - 1) / factor;
    (1..=q).map(|m| m * factor).collect()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .flat_map(|f| multiples(limit, *f))
        .collect::<HashSet<_>>()
        .iter()
        .sum()
}
