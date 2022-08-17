use std::io;

fn main() {
  pub const PI: f64 = 3.14;
  println!("Digite o raio: ");
  
    let mut leitura = String::new();

    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  let raio:f64 = leitura.trim().parse()
  .expect("Failed to convert");

  let comprimento = 2.0 * PI * raio;
  println!("Comprimento = {comprimento}");
  let area = PI * raio * raio;
  println!("Area = {area}");
  let volume = (3.0 * PI * raio * raio * raio)/4.0;
  println!("Volume = {volume}");

  
}