use crate::List::{Cons, Nil};
enum List
{
    Cons(i32, Box<List>),
    Nil,
}

fn print_list(list : &List)
{
    match list {
        Cons(value, next) => 
        {
            print!("{} -> ", value);
            print_list(next);
        },
        Nil => {
            println!("Nil");
        },


    }
}
fn main() 
{

    let list0 = Cons(1, Box::new(Cons(2, Box::new(Cons (3, Box::new(Nil))))));

    print_list(&list0);

    let test : i32 = 42;
    let ref_test = &test;
    let a = Box::new(5);
    let derefa = *a;
    println!("{}", &ref_test.0);

}
