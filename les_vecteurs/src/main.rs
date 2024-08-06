#[derive(Debug)]
enum Cellule
{
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {

    let mut vec_0: Vec<i32> = Vec::new();
    vec_0.push(12);
    println!("vec_0[0] = {}", &vec_0[0]);

    let mut vec_1 = vec![2,4,8,16,32];
    println!("vec_1 = {:?}", vec_1);

    for i in &mut vec_1
    {
        *i += 1;
        println!("{}", i);
    }

    let troisieme: &i32 = &vec_1[2];
    println!("vec_1[2] = {}", troisieme);

    match vec_1.get(90)
    {
        Some(&v) => println!("Le troisieme element est {}", &v),
        None => println!("Existe pas."),

    }

    let mut vec_2: Vec<String> = Vec::new();
    vec_2.push(String::from("Hello"));
    println!("vec_1[0] = {}", vec_2[0]); // ...
    println!("vec_1[0] = {}", &vec_2[0]);

    let vec_3 = vec![Cellule::Int(42), Cellule::Text(String::from("jtj")), Cellule::Float(12.3456)];
    match &vec_3[1]
    {
        Cellule::Int(value) => println!("{}", value),
        Cellule::Float(value) => println!("{}", value),
        Cellule::Text(value) => println!("{}", value),
    }

    let mut str_0 = String::new();
    let s : &str = "poupoupoupoauapou";
    str_0 = s.to_string();

    println!("{}", str_0);

    let mut str_1 = String::from("contenu initial");
    str_1.push_str(" foo bar");
    str_1.push('l');
    println!("{}", str_1);

    let str_2 = String::from("Hello ");
    let str_3 = String::from("World !");

    let str_4 = str_2 + &str_3;
    println!("{} {}", str_4, str_3);


    let str_5 = String::from("one");
    let str_6 = String::from("two");
    let str_7 = String::from("seven");

    let str_8 = str_5 + &str_6 + &str_7;

    println!("{}",str_8);

    let str_9 = format!("{} {} {}", str_8, str_6, str_7);

    println!("{}",str_9);
    println!("{}",str_8);

    let str_10 = "Здравствуйте";
    let str_11 = &str_10[0..6];

    println!("{}", str_11);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
