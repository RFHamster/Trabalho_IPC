use std::io;

fn main() {
  let mut cont = 0;
  let mut mediaAltura = 0;

  loop {
    println!("Digite a idade e a altura: ");
    let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let idade:i32 = leitura.trim().parse()
  .expect("Failed to convert");

    if idade <= 0 {
      break;
    }
    
    let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let altura:i32 = leitura.trim().parse()
  .expect("Failed to convert");
  
    if idade > 50 {
      mediaAltura += altura;
      cont += 1;
    }
    
  }
    
  let mediaFinal = mediaAltura/cont;
  println!("A media da altura eh: {mediaFinal}");
}