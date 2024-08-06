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

    let vec_1 = vec![2,4,8,16,32];
    println!("vec_1 = {:?}", vec_1);

    let troisieme: &i32 = &vec_1[2];
    println!("vec_1[2] = {}", troisieme);

    match vec_1.get(90)
    {
        Some(&v) => println!("Le troisieme element est {}", &vec_1[2]),
        None => println!("Existe pas."),

    }

    let mut vec_2: Vec<String> = Vec::new();
    vec_2.push(String::from("Hello"));
    println!("vec_1[0] = {}", vec_2[0]); // ...
    println!("vec_1[0] = {}", &vec_2[0]);

    let mut vec_3 = vec![Cellule::Int(42), Cellule::Text(String::from("jtj")), Cellule::Float(12.3456)];
    println!("vec_3 = {:?}", vec_3);


}
