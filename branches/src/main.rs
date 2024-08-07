fn main() {

    let condition = 12;

    if condition
    {
        println!("OK");
    }
    

    let nombre = 42;

    if nombre < 42
    {
        println!("<42");
    }
    else if nombre > 42
    {
        println!(">42");
    }
    else
    {
        println!("OK")
    }

    let variable : i32 = if condition {42} else {0};

    println!{"variable = {}", variable};

}
