use std::ops::Deref;
struct MaBoite<T>(T);

impl<T> MaBoite<T>
{
    fn new(valeur: T) -> MaBoite<T>
    {
        MaBoite(valeur)
    }
}

impl<T> Deref for MaBoite<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        // let j = &self.0;
        &self.0
    }
}

fn saluer(nom: &str) {
    println!("Salutations, {} !", nom);
}

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);


    let uneBoite = MaBoite::new(String::from("hello"));
    let oiopiu = &uneBoite;
    saluer(&uneBoite);
    saluer(&(*uneBoite));
    saluer(&(*uneBoite)[..]);
}
