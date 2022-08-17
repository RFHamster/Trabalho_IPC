use std::io;

fn main() {
  let mut vetorUm: Vec<f64> = Vec::new();
  let mut m: Vec<Vec<f64>>  = Vec::new();

  let mut l = 0;
  let mut c = 0;
  let mut media: f64 = 0.0;
  
  while l < 10 {
    c = 0;
    while  c < 10{
      println!("Digite M[{}][{}]: ", l, c);
      
      let mut leitura = String::new();

      io::stdin()
          .read_line(&mut leitura)
          .expect("Failed to read line");
  
      let leituraValor:f64 = leitura.trim().parse()
      .expect("Failed to convert");
  
      vetorUm.push(leituraValor);
      c += 1;
    }
    m.push((*vetorUm).to_vec());

    let mut index = 0;
    while index < 9 {
      vetorUm.remove(0);
      index += 1;
    }
    l += 1;
  }

  l = 0;
  while l < 10{
    c = 0;
    while c < 10{
      if l == c {
        media += m[l][c];
      }
      c += 1;
    }
    l += 1;
  }
  
  media = media/10.0;
  println!("Media dos números na diagonal: {}", media);
}