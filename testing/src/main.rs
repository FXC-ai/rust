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
    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..=range {
        optional_integers.push(Some(i));
    }

    println!("{:?}", optional_integers);

    let mut cursor = range;

    println!("{} {}", range, cursor);

}