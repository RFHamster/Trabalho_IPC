use std::io;

fn main() {

  println!("Digite 3 numeros em ordem crescente");
  
    let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let n1:i32 = leitura.trim().parse()
  .expect("Failed to convert");

  let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let n2:i32 = leitura.trim().parse()
  .expect("Failed to convert");

  let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let n3:i32 = leitura.trim().parse()
  .expect("Failed to convert");

  let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let n4:i32 = leitura.trim().parse()
  .expect("Failed to convert");

  if n4 > n3{
      println!("A ordem decrescente é: {n4} - {n3} - {n2} - {n1}");
    }else if n4 > n2 && n4 < n3 {
      println!("A ordem decrescente é: {n3} - {n4} - {n2} - {n1}");
    }else if n4 > n1 && n4 < n2 {
      println!("A ordem decrescente é: {n3} - {n2} - {n4} - {n1}");
    }else if n4 < n1 {
      println!("A ordem decrescente é: {n3} - {n2} - {n1} - {n4}");
    }

}