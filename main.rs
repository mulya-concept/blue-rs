// add library
use std::io::{self};
mod persegi;

fn main(){
    println!("tentukan nilai sisi : ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("gagal membaca input");
    let sisi:f64 = input.trim().parse().expect("input harus berupa angka");

    // 
    let luas: f64 = persegi::luas_persegi(sisi);
    let _keliling: f64 = persegi::keliling_persegi(sisi);

    println!("luas persegi : {}", luas);
}