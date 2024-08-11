struct Point <T, U>
{
    x: T,
    y: U,
}

impl<T, U> Point<T, U>
{
    fn x(&self) -> &T
    {
        &self.x
    }
    fn y(&self) -> &U
    {
        &self.y
    }

    fn set_x(&mut self, value : T)
    {
        self.x = value;
    }
    
    fn set_y(&mut self, value : U)
    {
        self.y = value;
    }

}


impl Point <i64, i64>
{
    fn get_x_i64(&self) -> &i64
    {
        &self.x
    }

    fn get_y_i64(&self) -> &i64
    {
        &self.y
    }
}


impl Point <i32, i32>
{
    fn get_Xi32(self) -> i32
    {
        self.x
    }
    fn get_Yi32(self) -> i32
    {
        self.y
    }
    fn add_coord(&self) -> i32
    {
        &self.x + &self.y
    }
}

impl Point <String, &str>
{
    fn get_XString(&self) -> &String
    {
        &self.x
    }

    fn get_Ystr(&self) -> &str
    {
        self.y
    }
}

impl<U> Point <&String, U>
{
    fn get_XrefString(&self) -> &String
    {
        self.x
    }
}

impl<X1, Y1> Point<X1, Y1> 
{

    fn set_X(&mut self, value : X1)
    {
        self.x = value;
    }

    fn get_X(&self) -> &X1
    {
        &self.x
    }

    fn get_Y(&self) -> &Y1
    {
        &self.y
    }
}

// impl<X1, Y1> Point<X1, Y1>
// {
//     fn echange_coord<X2, Y2>(&self, other: &mut Point<X2, Y2>)
//     {
//         let x1 = &self.x;
//         let y1 = &self.y;

//         Point
//         {
//             &self.x = other.x;
//             &self.y = other.y;
//             &mut other.x = x1;
//             &mut other.y = y1;
//         }
//     }

// }


fn main()
{
    let p0 = Point{x:5,y:10};
    let p1 = Point{x:5.0,y:10};
    let p2 = Point{x:5,y:10.0};
    let mut p3 = Point{x:5.0,y:10.0};

    println!("{} {}", p2.x(), p2.y());
    println!("{}", p0.add_coord());

    p3.set_X(42.0);
    println!("{} {}", p3.get_X(), p3.get_Y());
    println!("{} {}", p3.x, p3.y);

    println!("{} {}", p0.x(), p0.y());
    println!("{}", p0.get_Xi32());
    // println!("{} {}", p0.x, p0.y);
    
    let p4 = Point{x:String::from("Jeu de go"), y:"tu trois hp"};
    println!("{}", p4.get_XString());
    println!("{}", p4.x);
    println!("{}", p4.get_Ystr());
    println!("{}", p4.y);

    let mut p5 = Point{x:&String::from("loi des grands"), y:"pagaille du petit"};
    println!("{}", p5.get_XrefString());
    println!("{}", p5.x());
    // p5.x = &String::from("oil sed sdnarg");
    let binding = String::from("ta tatie t'a tout dit");
    p5.set_x(&binding);
    println!("{}", p5.x);
    println!("{}", p5.y());

    let p6 = Point{x:269000000, y:32000000};
    println!("{}", p6.get_x_i64());
}