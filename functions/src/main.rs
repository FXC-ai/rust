fn main() {
    let y = {
        let x = 41;
        x + 1
    };
    print_param(y);


    let test = cinq();
    println!("cinq() = {}", test);
    
    println!("plus_one(41) = {}", plus_one(42));



}

fn cinq() -> i32
{
    5
}

fn print_param(x : i32)
{
    println!("L'argument du parametre x est {}.", x);
}

fn plus_one(x : i32) -> i32
{
    if x == 42
        return 42
    x + 1
}