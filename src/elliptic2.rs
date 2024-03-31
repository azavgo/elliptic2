use std::f64::consts::PI; 
use crate::{factorial::*, power::*};

fn element(k: f64, n: usize) -> f64 {
    let a = (double_factorial(2*n-1)/double_factorial(2*n)) as f64;
    a * a * power(k, 2*n)/((2*n - 1) as f64)
}

//TODO: Seems incorrect results
pub fn elliptic2(k: f64) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::with_capacity(5); 
    let mut _e: f64 = 0.0;
    for i in 1..=5 {
        _e = (PI/2.0_f64)*(1.0_f64 - element(k, i));
        vec.push(_e);
    }
    return vec;
}
