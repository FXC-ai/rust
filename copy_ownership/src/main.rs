fn main()
{
    let s = hello();
    println!("{}", s);
}

fn hello() -> String
{
    let s = String::from("hello");
    s
}