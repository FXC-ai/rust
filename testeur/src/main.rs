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
    
}

impl Point <i32, i32>
{
    fn add_coord(&self) -> i32
    {
        &self.x + &self.y
    }

}

impl<X1, Y1> Point<X1, Y1>
{
    fn echange_coord<X2, Y2>(&self, other: &mut Point<X2, Y2>)
    {
        let x1 = &self.x;
        let y1 = &self.y;

        Point
        {
            &self.x = other.x;
            &self.y = other.y;
            &mut other.x = x1;
            &mut other.y = y1;
        }
    }

}


fn main()
{
    let p0 = Point{x:5,y:10};
    let p1 = Point{x:5.0,y:10};
    let p2 = Point{x:5,y:10.0};
    let p3 = Point{x:5.0,y:10.0};

    println!("{} {}", p2.x(), p2.y());
    println!("{}", p0.add_coord());

    


}

