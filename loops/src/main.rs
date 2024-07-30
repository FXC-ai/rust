fn main()
{
    // let mut c : i32 = 2;
    // let nb = 42;

    // let premier_diviseur = loop 
    // {
    //     if nb % c == 0{break c;}
    //     c += 1;
    // };

    // println!("{}", premier_diviseur);

    let nb = 53;

    for premier_diviseur in 2..=nb
    {
        if nb % premier_diviseur == 0 {println!("{}", premier_diviseur); break;}
    } 

    let tab = [12, 13 ,14 ,15];

    for nb in tab
    {
        println!("{}", nb);
    }

}
