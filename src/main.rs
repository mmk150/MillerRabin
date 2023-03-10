#![allow(unused)]

use rand::Rng;
use rayon::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, ErrorKind, Write};
use std::path::Path;
use std::path::PathBuf;

fn decomp(integer: u64) -> (u64, u64) {
    let mut s: u64 = 0;
    let mut number: u64 = integer - 1;

    while number % 2 == 0 {
        number /= 2;
        s += 1;
    }
    return (s, number);
}

fn powmod(base: u64, exponent: u64, modulus: u64) -> u64 {
    if exponent == 0 {
        return 1;
    }
    if exponent == 1 {
        return base;
    }
    if exponent == 2 {
        return base * base % modulus;
    }

    let mut float_exp: f64 = exponent as f64;
    let mut s: f64 = float_exp.log2().floor();

    let mut s = s as u64;
    let mut two_s: u64 = 2;
    two_s = two_s.pow(s as u32);

    let mut d = exponent - two_s;
    let mut baseholder = base;

    for index in 0..s {
        baseholder = baseholder * baseholder % modulus;
    }

    return baseholder * powmod(base, d, modulus) % modulus;
}

fn is_mr_witness(candidate: u64, number: u64, s: u64, d: u64) -> bool {
    let mut s: u64 = s;
    let mut d: u64 = d;
    let mut init: u64 = powmod(candidate, d, number);

    if init == 1 {
        return false;
    }

    for i in 0..s {
        if init == number - 1 {
            return false;
        }
        init = powmod(init, 2, number);
    }

    return true;
}

fn check_prime(x: u64) -> bool {
    let pvec: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    let mut s: u64;
    let mut d: u64;
    (s, d) = decomp(x);
    let res: bool = false;
    for p in pvec {
        let res = is_mr_witness(p, x, s, d);
        if res == true {
            return false;
        }
    }

    return true;
}

fn non_witness_set_x(number: u64) {
    let isprime: bool = check_prime(number);

    let mut s: u64;
    let mut d: u64;

    (s, d) = decomp(number);
    //Get decomposition N-1=d*2^s
    let mut path = PathBuf::from("./witnesses/");
    path.push(&number.to_string());
    path.set_extension("txt");

    //dbg!(&path);
    let mut file = OpenOptions::new()
        .write(true)
        //.append(true)
        .create(true)
        .open(path)
        .expect("fail to open file");

    //Begin check
    let mut writer = BufWriter::new(file);
    if isprime {
        writeln!(writer, "prime");
        writer.flush();
        return;
    }

    for i in 2..number - 1 {
        let mut res: bool = is_mr_witness(i, number, s, d);
        if res == false {
            writeln!(writer, "{},", i);
            //count+=1;
        }
    }
    //return count;
    writer.flush();
}

fn non_witness_range(lower_bound: u64, upper_bound: u64) {
    let mut vec: Vec<u64> = (lower_bound..=upper_bound).step_by(2).collect();
    vec.par_iter().for_each(|x| non_witness_set_x(*x));
}

fn main() {
    let a: u64 = 1_000_601;
    let b: u64 = 1_003_639;

    non_witness_range(a, b);
}
