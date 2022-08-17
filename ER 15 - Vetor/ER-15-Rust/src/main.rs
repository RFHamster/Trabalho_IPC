use std::io;

fn main() {
    
  let mut vetorUm: Vec<i32> = Vec::new();
  let mut vetorDois: Vec<i32> = Vec::new();
  let mut i = 0;
  let mut j = 0;
  let mut possuiDivisor = 0;
  
  println!("Vetor 1");
  while i < 10 {
    println!("V[{}]: ", i+1);
    
    let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let leituraValor:i32 = leitura.trim().parse()
  .expect("Failed to convert");

  vetorUm.push(leituraValor);
    
    i += 1;
  }
  i = 0;
  
  println!("Vetor 2");
  while i < 5 {
    println!("V[{}]: ", i+1);
    
    let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let leituraValor:i32 = leitura.trim().parse()
  .expect("Failed to convert");

  vetorDois.push(leituraValor);
    
    i += 1;
  }
  i = 0;

  print!("== Vetor 1: ");
  while i < 10 {
    print!("{} ", vetorUm[i]);
    i += 1;
  }
  i = 0;
  println!();
  
  print!("== Vetor 2: ");
  while i < 5 {
    print!("{} ", vetorDois[i]);
    i += 1;
  }
  i = 0;
  println!();

  while i < 10 {
    println!("");
    println!("Numero {}", vetorUm[i]);
    possuiDivisor = 0;
    while j < 5 {
      if vetorUm[i] % vetorDois[j] == 0 {
        println!("Divisivel por {} na posicao {}", vetorDois[j], (j + 1));
        possuiDivisor = 1;
      }
      j += 1;
    }
    j = 0;
    if possuiDivisor == 0 {
      println!("Nao possui Divisor");
    }
    i += 1;
  }
  i = 0;
}