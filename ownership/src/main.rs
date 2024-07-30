fn main() {
    let s1 = donne_possession();
  
    let s2 = String::from("hello");
  
    let s3 = prend_et_rend(s2);

    println!("{} {}", s1, s3);
}
  
fn donne_possession() -> String 
{
    let texte = String::from("yours");
    texte
}

fn prend_et_rend(texte: String) -> String {
    texte
}