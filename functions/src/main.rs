fn main() {
    let y = {
        let x = 41;
        x + 1
    };
    une_autre_fct(y);


    let test = cinq();
    println!("cinq() = {}", test);



}

fn cinq() -> i32
{
    5
}

fn une_autre_fct(x : i32)
{
    println!("L'argument du parametre x est {}.", x);
}
