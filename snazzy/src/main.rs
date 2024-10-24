use snazzy::create_large_shirt;

fn main()
{
    println!("test");

    let my_shirt = create_large_shirt("rouge".to_string());

    println!("{:?}", my_shirt.size());
}