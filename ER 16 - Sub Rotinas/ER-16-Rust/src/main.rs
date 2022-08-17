use std::io;

fn main() {
  let mut A: Vec<i32> = Vec::new();
  let mut B: Vec<i32> = Vec::new();
  let mut C: Vec<i32> = Vec::new();
  
  let mut i = 0;
  while i < 10 {
    println!("Digite A[{}]", i+1);
    let mut leitura = String::new();
    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

    let numero:i32 = leitura.trim().parse()
    .expect("Failed to convert");
    
    A.push(numero);
    i = i + 1;
  }
  
  i = 0;
  while i < 10 {
    println!("Digite B[{}]", i+1);
    let mut leitura = String::new();
    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

    let numero:i32 = leitura.trim().parse()
    .expect("Failed to convert");
    
    B.push(numero);
    i = i + 1;
  }

  let a = &mut A;
  let b = &mut B;
  let c = &mut C;
  arrumaVetor(a, b, c);

  println!("");
  print!("Vetor C: ");
  i = 0;
  while i < 20 {
    print!("{} ", C[i]);
    i = i + 1;
  }
}


fn arrumaVetor(A: &mut Vec<i32>, B: &mut Vec<i32>, C: &mut Vec<i32>){
  let mut i = 0;
  while i < 10 {
    C.push(A[i]);
    i = i + 1;
  }
  i = 0;
  while i < 10 {
    C.push(B[i]);
    i = i + 1;
  }

  let mut maior = 0;
  let mut j = 0;
  i = 0;
  while i < 20 {
    j = i+1;
    while j < 20 {
      if C[i] < C[j] {
        maior = C[j];
        C[j] = C[i];
        C[i] = maior;
      }
      j += 1;
    }
    i += 1;
  }
}