mod cellule
{
    pub trait fct_cellulaire
    {
        fn get_name(&self) -> &str;
        fn set_name(& mut self, new_name : &str);

        fn action(&self) -> i32;

        fn default_action(&self)
        {
            println!("My name is {}.", self.get_name());
        }
    }

    pub struct mitochondrie
    {
        pub name : String,
    }

    pub struct ribosome
    {
        pub name : String,
    }

    impl fct_cellulaire for mitochondrie
    {
        fn get_name(&self) -> &str
        {
            &self.name
        }
        fn set_name(&mut self, new_name : &str)
        {
            self.name = new_name.to_string();
        }
        fn action(&self) -> i32
        {
            42
        }
    }
    
    impl fct_cellulaire for ribosome
    {
        fn get_name(&self) -> &str {
            &self.name
        }
        fn set_name(&mut self, new_name : &str)
        {
            self.name = new_name.to_string(); // ?
        }
        fn action(&self) -> i32
        {
            21
        }    
    }
}

fn bring_action(element : &impl fct_cellulaire) -> i32
{
    element.action()
}

fn bring_action_sans_sucre<T: fct_cellulaire>(element : &T) -> i32
{
    element.action()
}

fn bring_actions<T: fct_cellulaire>(element0 : &T, element1 : &T) -> i32
{
    element0.action() + element1.action()
}

fn bring_actions_2(element0 : &impl fct_cellulaire, element1 : &impl fct_cellulaire) -> i32
{
    element0.action() + element1.action()
}

use std::fmt::Display;

use cellule::{fct_cellulaire, mitochondrie, ribosome};

fn main()
{
    let mut mitochondrie_0 = mitochondrie{name : String::from("mitochondrie_0")};

    println!("{}", mitochondrie_0.get_name());

    mitochondrie_0.set_name("mitochondrie");

    println!("{}", mitochondrie_0.get_name());

    let mut ribosome_0: ribosome = ribosome{name : String::from("ribosome_0")};

    println!("{}", ribosome_0.get_name());

    ribosome_0.set_name("ribosome");
    
    println!("{}", ribosome_0.get_name());

    ribosome_0.default_action();

    println!("{}", bring_action(&ribosome_0));

    println!("{}", bring_action_sans_sucre(&ribosome_0));

    println!("{}", bring_actions(&ribosome_0, &ribosome_0));

    println!("{}", bring_actions_2(&ribosome_0, &mitochondrie_0));

}