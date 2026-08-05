// Alterem para receber um número e mostrar a tabuada do numero

use std::io;

fn main() {
  let mut input = String::new();
  
  println!("Digite um numero para ver a tabuada: ");
  
  io::stdin().read_line(&mut input).unwrap();
      
  let numero: i32 = input.trim().parse().unwrap(); 

  for i in 1..=10 {
    println!("{} * {} = {}", i, numero, i * numero);
  }
}

//
