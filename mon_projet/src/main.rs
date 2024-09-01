struct colleague
{
    name : String,
    company : String,
}
struct friend
{
    name : String,
    origin : String,
}
trait friend_abilities
{
    fn get_name_friend(&self) -> &str;
    fn set_name_friend(&mut self, new_name : String);
    fn get_origin(&self) -> &str;
}

impl friend_abilities for friend
{
    fn get_name_friend(&self) -> &str {
        &self.name
    }
    fn set_name_friend(&mut self, new_name : String) {
        self.name = new_name;
    }
    fn get_origin(&self) -> &str {
        &self.origin
    }
}

impl friend_abilities for colleague
{
    fn get_name_friend(&self) -> &str {
        &self.name
    }
    fn set_name_friend(&mut self, new_name : String) {
        self.name = new_name;
    }
    fn get_origin(&self) -> &str {
        &self.company
    }
}
trait colleague_abilities
{
    fn get_name_colleague(&self) -> &str;
    fn set_name_colleague(&mut self, new_name : String);
    fn get_company(&self) -> &str;
}

impl colleague_abilities for colleague
{
    fn get_name_colleague(&self) -> &str {
        &self.name
    }
    fn set_name_colleague(&mut self, new_name : String) {
        self.name = new_name;
    }
    fn get_company(&self) -> &str {
        &self.company
    }
}

impl colleague_abilities for friend
{
    fn get_name_colleague(&self) -> &str {
        &self.name
    }
    fn set_name_colleague(&mut self, new_name : String) {
        self.name = new_name;
    }
    fn get_company(&self) -> &str {
        &self.origin
    }
}

fn show <T : colleague_abilities + friend_abilities>(element : &T)
{
    println!("{} {} {} {}", element.get_name_colleague(), element.get_name_friend(), element.get_company(), element.get_origin());
}

enum sports
{
    running,
    climbimg,
    skiTouring,
}
trait other_abilities
{
    fn sport_capacity (sport : sports) -> String;
}


impl other_abilities for friend
{
    fn sport_capacity (sport : sports) -> String
    {
        match sport
        {
            sports::running => {"Alexandre Boucheix".to_string()},
            sports::climbimg => {"Chris Charma".to_string()},
            sports::skiTouring => {"Lena Bonel".to_string()}
        }
    }

}


fn main()
{

    let mut  Alice = colleague{name : String::from(""), company : String::from("Mairie")};
    Alice.set_name_colleague(String::from("Alice"));
    show(&Alice);
    let Duncan = friend{name : String::from("Duncan"), origin : String::from("Martigny")};
    show(&Duncan);

}
