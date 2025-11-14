use ark_bn254::Fr;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn sets_and_cartesian_products() {
    println!("--- 1. Sets and Cartesian Products ---");

    // Define two sets over field elements
    let set_a: Vec<Fr> = (0..3).map(Fr::from).collect();   // {0,1,2}
    let set_b: Vec<Fr> = (2..5).map(Fr::from).collect();   // {2,3,4}

    println!("Set A: {:?}", set_a);
    println!("Set B: {:?}", set_b);

    // --- Union ---
    let union = set_union(&set_a, &set_b);
    println!("A ∪ B = {:?}", union);

    // --- Intersection ---
    let intersection = set_intersection(&set_a, &set_b);
    println!("A ∩ B = {:?}", intersection);

    // --- Difference ---
    let difference = set_difference(&set_a, &set_b);
    println!("A \\ B = {:?}", difference);

    // --- Power Set ---
    let power_set_a = power_set(&set_a);
    println!("|P(A)| = {} (Power set of A)", power_set_a.len());

    // --- Cartesian Product ---
    let cartesian = cartesian_product(&set_a, &set_b);
    println!("A × B has {} pairs", cartesian.len());
    println!("Sample of A × B: {:?}", &cartesian[0..std::cmp::min(5, cartesian.len())]);
}

// ===============================================================
// Helper Functions for Set Operations
// ===============================================================

// Union: A ∪ B
fn set_union<T: Clone + Eq + std::hash::Hash>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut set: HashSet<T> = HashSet::from_iter(a.iter().cloned());
    set.extend(b.iter().cloned());
    set.into_iter().collect()
}

// Intersection: A ∩ B
fn set_intersection<T: Clone + Eq + std::hash::Hash>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let set_a: HashSet<T> = HashSet::from_iter(a.iter().cloned());
    let set_b: HashSet<T> = HashSet::from_iter(b.iter().cloned());
    set_a.intersection(&set_b).cloned().collect()
}

// Difference: A \ B
fn set_difference<T: Clone + Eq + std::hash::Hash>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let set_a: HashSet<T> = HashSet::from_iter(a.iter().cloned());
    let set_b: HashSet<T> = HashSet::from_iter(b.iter().cloned());
    set_a.difference(&set_b).cloned().collect()
}

// Power Set: P(A)
fn power_set<T: Clone>(a: &Vec<T>) -> Vec<Vec<T>> {
    let n = a.len();
    let mut result = Vec::new();
    for mask in 0..(1 << n) {
        let subset: Vec<T> = (0..n)
            .filter(|i| (mask & (1 << i)) != 0)
            .map(|i| a[i].clone())
            .collect();
        result.push(subset);
    }
    result
}

// Cartesian Product: A × B
fn cartesian_product<T: Clone, U: Clone>(a: &Vec<T>, b: &Vec<U>) -> Vec<(T, U)> {
    a.iter()
        .flat_map(|x| b.iter().map(move |y| (x.clone(), y.clone())))
        .collect()
}

// ===============================================================
// Entry Point for Testing
// ===============================================================
fn main() {
    sets_and_cartesian_products();
}
