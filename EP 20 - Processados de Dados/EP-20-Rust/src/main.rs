use std::fs::File;
use std::io::prelude::*;

fn main() {
    //println!("In file {}", produto.txt);
  let mut letraM = 0;

    let mut f = File::open("produto.txt").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");


  let vetor = contents.split("\n");
  for nova in vetor {
    letraM = 0;
    for b in nova.bytes(){
      if b == 109 || b == 77 {
        letraM = 1;
      }
    }
    if letraM == 1 {
      println!("{nova}");
    }
  }
}
