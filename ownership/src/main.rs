struct Cercle
{
    id: u32,
    rayon: f32,
}

impl Cercle
{
    fn aire(self : &Self) -> f32 // equivalent à &self 
    {
        self.rayon * 3.14 * 2.0
    }

    fn change_id(self : &mut Self, id : u32) // equivalent à &mut self
    {
        self.id = id;
    }

    fn change_owner(self : Self) -> Cercle // equivalent à self
    {
        Self{id : self.id, rayon : self.rayon}
    }

    fn able_fit_in(&self, other : &Cercle) -> bool
    {
        self.aire() > other.aire()
    }

    fn create_cercle (id : u32, rayon : f32) -> Cercle
    {
        Cercle {id : id, rayon : rayon}
    }

}

fn main() {
    let mut cercle0 = Cercle {id:0, rayon:5.0};

    (&mut cercle0).change_id(42);
    println!("{} Surface du cercle = {}",cercle0.id, cercle0.aire());

    let cercle1 = cercle0.change_owner();
    println!("Surface du cercle = {}", cercle1.aire());
    
    let cercle2 = Cercle {id : 2, rayon:8.8};

    println!("Cercle2 peut contenir cercle1 : {}", cercle2.able_fit_in(&cercle1));
    println!("Cercle1 peut contenir cercle2 : {}", cercle1.able_fit_in(&cercle2));

    let cercle3 = Cercle::create_cercle(3, 5.6);
    println!("Surface du cercle = {}", cercle3.aire());

}
