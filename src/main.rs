


#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, BufWriter,ErrorKind};
use std::path::PathBuf;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;
use rayon::prelude::*;
use primes::{Sieve,PrimeSet};




fn decomp(integer:u64)-> (u64,u64){
  let mut s:u64=0;
  let mut number:u64=integer-1;

  while number % 2 ==0{
    number/=2;
    s+=1;
  }
  return (s,number);
}

fn powmod(base:u64, exponent:u64, modulus: u64)-> u64{
  if exponent==0{
    return 1;
  }
  if exponent==1{
    return base;
  }
  if exponent==2{
    return base*base % modulus;
  }
  
    let mut float_exp:f64= exponent as f64;
    let mut s:f64=float_exp.log2().floor();

    let mut s =s as u64;
    let mut two_s:u64= 2;
    two_s=two_s.pow(s as u32);

    let mut d = exponent-two_s;
    let mut baseholder=base;
   

    for index in 0..s{
      baseholder=baseholder*baseholder % modulus;
    }
   
    return baseholder*powmod(base,d,modulus) % modulus;
    
  }


  fn unitTest(){
    let mut v_ans: Vec<u64> = Vec::new();
    let mut ans:u64;
    
    let v_base:Vec<u64> = vec![347,204,380,416,449,369,20200010000];
    let mut base:u64=12;
  
    let v_exp:Vec<u64> = vec![236,321,488,31,146,417,1220314060];
    let mut exp:u64=99;
  
    let v_mod:Vec<u64> = vec![176,79,59,230,472,314,252009990000];
    let mut modulus:u64=200;
    for index in 0..7{
      base=v_base[index];
      exp=v_exp[index];
      modulus=v_mod[index];
      ans=powmod(base,exp,modulus);
      v_ans.push(ans);
    }
  for answers in v_ans{
    println!("Ans is {}", answers)
  }
   
  }

  fn is_mr_witness(candidate:u64,number:u64,s:u64,d:u64)-> bool {
    let mut s:u64=s;
    let mut d:u64=d;
    let mut init:u64=powmod(candidate,d, number);

    if init==1 {
      return false;
    }

    for i in 0..s{
      if init==number-1 {
        return false;
      }
      init=powmod(init,2,number);  
    }
   
    
    return true;
  }


  fn non_witness_set_x(number: u64,isprime:bool) {


    let mut s:u64;
    let mut d:u64;
    //let mut count:i128=0;
    (s,d)=decomp(number);
    //Get decomposition N-1=d*2^s
    let mut path = PathBuf::from("./witnesses/");
    path.push(&number.to_string());
    path.set_extension("txt");

    dbg!(&path);
    let mut file=OpenOptions::new()
      .write(true)
      //.append(true)
      .create(true)
      .open(path)
      .expect("fail to open file");

    
    //Begin check
    let mut writer=BufWriter::new(file);
    if isprime{
      writeln!(writer,"prime");
      writer.flush();
      return;
    }
    for i in 2..number-1 {
      let mut res:bool= is_mr_witness(i,number,s,d);
      if res==false{
       writeln!(writer,"{},",i);
        //count+=1;
      }
    }
    //return count;
    writer.flush();

  
  }

  fn non_witness_range(lower_bound:u64,upper_bound:u64,mut pset:Sieve){
    fn isprime(x:u64,mut pset:Sieve)-> bool{
      return pset.is_prime(x);
    }
    let mut vec: Vec<u64>=(lower_bound..=upper_bound).step_by(2).collect();
    vec.par_iter()
    .for_each(|x| {
      let isprime=isprime(*x,pset);
      non_witness_set_x(*x,isprime)});
  }

  


fn main(){
  
  let mut pset=Sieve::new();  
  let (p1,p2) =pset.find(1_000_000);
  
  let a:u64=19_000_601;
  let b:u64=19_000_639;
  non_witness_range(a,b, pset);


}
