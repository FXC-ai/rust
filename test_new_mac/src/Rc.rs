use crate::List::{Cons, Nil};
use std::rc::Rc;



enum List {
    Cons(i32, Rc<List>),
    Nil

}

fn main ()
{
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    let b = Cons(3, Rc::clone(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("{}", Rc::strong_count(&a));
    }
    println!("{}", Rc::strong_count(&a));

}