use std::io;

fn main() {
  let mut maiuscula = 1;
  let mut bytes: Vec<u8> = Vec::new();
  let mut leitura = String::new();
    io::stdin()
        .read_line(&mut leitura)
        .expect("Failed to read line");

  for b in leitura.bytes() {
    let mut byte = b;
    if byte == 32 {
      bytes.push(byte);
      maiuscula = 1;
    }else{
      if maiuscula == 1{
        if byte >= 97 {
          byte = byte - 32;
        bytes.push(byte);
        }
        maiuscula = 0;
      }else {
        bytes.push(byte);
      }
    } 
  }
 
  let s = String::from_utf8(bytes).expect("Found invalid UTF-8");
  println!("{}", s);
}