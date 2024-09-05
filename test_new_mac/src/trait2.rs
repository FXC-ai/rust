use std::fmt::Display;

fn donne_s0 <'a>(st0 : &'a str, st1 : &  str) -> &'a str
{
    println!("{} : ", st1);
    st0
}

fn la_plus_longue<'a>(st0 : &'a str, st1 : &'a str) ->&'a str
{
    if st0.len() > st1.len()
    {
        st0
    }
    else
    {
        st1
    }
}
struct MyStruct<'a>
{
    s: &'a str,
}

impl<'a> MyStruct<'a>
{
    fn niveau (&self) -> i32
    {
        42
    }
    fn announce_and_display(&self, announcement: &str) -> &str
    {
        println!("{}", announcement);
        self.s
    }
}

fn fct_ex<'a, T : Display>(x: &'a str, y:&'a str, ann : T) -> &'a str
{
    println!("{}", ann);
    if x.len() > y.len()
    {
        x
    }
    else
    {
        y
    }
}
fn main()
{
    let st0 = String::from("hello");

    let resultat;

    {
        let st1 = String::from("donne_s0");
        resultat = donne_s0(st0.as_str(), st1.as_str());
    }
    println!("{}", resultat);
    
    println!("---------------------------------------------------------");

    {
        let s0 = String::from("Hellofff");

        {
            let s1 = String::from("World");
            let r = la_plus_longue(s0.as_str(), s1.as_str());
            println!("{}", r);
        }
    }
    println!("---------------------------------------------------------");

    {
        let s0 = String::from("Hellofff");
        let s1 = String::from("World");

        {
            let r = la_plus_longue(s0.as_str(), s1.as_str());
            println!("{}", r);
        }        
    }

    println!("---------------------------------------------------------");

    // let s = String::from("double 2 c est mieux");
    let uneMyStruct = MyStruct{s:"double 1 c est moins bien"};
    println!("{}", uneMyStruct.s);
    println!("{}", uneMyStruct.niveau());
    let r = uneMyStruct.announce_and_display("Hello");
    println!("{}", r);

    println!("---------------------------------------------------------");

    let s: &'static str = "le toit de la maison";

    println!("{}", s);

    println!("---------------------------------------------------------");

    fct_ex("x", "y", "ann");


}