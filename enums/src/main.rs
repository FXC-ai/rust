enum Message
{
    Quitter,
    Deplacer { x: i32, y: i32 },
    Ecrire(String),
    ChangerCouleur(i32, i32, i32),
}

impl Message 
{
    fn appeler(&self) 
    {
        println!("Hello !");
    }
}

fn main() 
{
    let m = Message::Ecrire(String::from("hello"));
    m.appeler();

    let c = Message::Quitter;
    c.appeler();

    let u : Option<i32> = None;

    let v = Some(5);
    let y = Some(6);

    let z = v +y;
}
