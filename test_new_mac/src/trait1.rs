enum Note<T>
{
    Tresbien(T),
    Bien(T),
    Assezbien(T),
    Passable(T),
    Mal(T),
    Tresmal(T),
}

impl <T : Display> Note<T>
{
    fn appreciation (&self)
    {
        match self
        {
            Note::Tresbien(value) => {println!("{}", value)},
            Note::Bien(value) => {println!("{}", value)},
            Note::Assezbien(value) => {println!("{}", value)},
            Note::Passable(value) => {println!("{}", value)},
            Note::Mal(value) => {println!("{}", value)},
            Note::Tresmal(value) => {println!("{}", value)},
        }
    }
}
fn main()
{
    let note0 = Note::Tresbien(6);
    note0.appreciation();
}