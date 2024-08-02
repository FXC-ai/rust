struct User
{
    actif: bool,
    pseudo: String,
    email: String,
    nombre_de_connexions: u64,
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn affiche_color(color : Color)
{
    println!("color = {} {} {}", color.0, color.1, color.2);
}

fn affiche_point(point : Point)
{
    println!("point = {} {} {}", point.0, point.1, point.2);
}

fn create_user(email: String, pseudo: String) -> User
{
    User
    {
        email: email,
        pseudo: pseudo,
        actif: true,
        nombre_de_connexions: 0,
    }
}

fn create_user_raccourci(email: String, pseudo: String) -> User
{
    User
    {
        email,
        pseudo,
        actif: true,
        nombre_de_connexions: 0,
    }
}

struct ToujoursEgal;

fn main()
{

    let user0 = User
    {
        email: String::from("user0@rust.com"),
        pseudo: String::from("user0"),
        actif: true,
        nombre_de_connexions : 54,
    };

    println!("{}", user0.pseudo);

    let mut user1 = create_user(String::from("user1@rust.com"), String::from("user1"));

    user1.email = String::from ("user1@rust.ch");

    println!("{} {}", user1.pseudo, user1.email);

    let user4 = create_user_raccourci(String::from("user4@rust.com"), String::from("user4"));

    println!("{}", user4.pseudo);

    let user2 = User
    {
        email: String::from("user2@rust.com"),
        ..user1
    };
    println!("{} {}", user2.pseudo, user2.email);

    let user3 = User
    {
        email : String::from("user3@rust.com"),
        pseudo : String::from("user3"),
        ..user2
    };
    println!("{} {}", user2.pseudo, user2.email);
    println!("{} {}", user3.pseudo, user3.email);


    let mon_point = Point(0,1,2);

    affiche_point(mon_point);

    let ma_couleur = Color(193,10,255);

    affiche_color(ma_couleur);

    let sujet = ToujoursEgal;

}

