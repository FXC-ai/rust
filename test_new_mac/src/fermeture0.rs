fn  ajouter_un_v1   (x: u32) -> u32 { x + 1 }


fn main()
{
    let mut y = 20;
    let mut x = 10;
    let z = 5;
    let mut assigner_y  = || { y = x;};
    
    assigner_y();

    let ajouter_un_v3 = |x|{ x + y };

    println!("{}", ajouter_un_v3(3));

    let ajouter_un_v4 = |x| x + y;

    println!("{}", ajouter_un_v4(3));

    let mut a = String::from("poire");

    let assigner_a = move |b : &str| -> String {a.push_str(b);a};

    let l = assigner_a(" poirE");

    println!("{}", l);

}