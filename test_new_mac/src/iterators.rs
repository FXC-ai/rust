#[derive(Debug)]
struct nawak 
{
    valeur: i32,
}

impl Iterator for nawak 
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.valeur > -21474838
        {
            self.valeur -= 1;
            Some(self.valeur)
        }
        else 
        {
            None
        }
    }
}

fn main()
{

    let mut myNawak_iter = nawak{valeur : -2147};
    println!("{:?}", myNawak_iter);

    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();
    myNawak_iter.next();


    println!("{:?}", myNawak_iter);

    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    for val in v1_iter
    {
        println!("On peut faire mieux que ça : {}", val);
    }

    let v2 = vec![1, 2, 3];

    let mut v2_iter = v2.iter();
    println!("{:?}", v2_iter);

    assert_eq!(v2_iter.next(), Some(&1)); 
    println!("{:?}", v2_iter);

    v2_iter.next();
    println!("{:?}", v2_iter);

    v2_iter.next();
    assert_eq!(v2_iter.next(), None); 
    println!("{:?}", v2_iter); 

    v2_iter.next();
    assert_eq!(v2_iter.next(), None); 
    println!("{:?}", v2_iter);

    let mut un = 1;
    let mut emprunt = &mut un;
    *emprunt += 1;
    println!("{}", un);

    let mut v3 = vec![1,2,3];

    println!("{:?}", v3);

    let mut v3_iter = v3.iter_mut();

    for val in v3_iter
    {
        *val += 1;
    }

    println!("{:?}", v3);

    let mut v4 = vec![1,2,3];
    println!("{:?}", v4);

    let mut v4_iter = v4.into_iter();
    println!("{:?}", v4_iter);

    println!("{:?}", v4_iter.next());
    println!("{:?}", v4_iter.next());
    println!("{:?}", v4_iter.next());
    println!("{:?}", v4_iter.next());

    println!("{:?}", v4_iter);
    
    let v5 = vec![1,2,3];

    let v5_iter = v5.iter();

    let tot : i32 = v5_iter.sum();

    println!("{}", tot);

    let v6: Vec<i32> = vec![1, 2, 3];

    let v7 = v6.iter().map(|x| x + 1000);
    println!("{:?}", v7);


    let v8 : Vec<_> = v7.collect();
    println!("{:?}", v8);

    let v9 = vec![10,2,36];

    let v10 = v9.iter().filter(|x| x > &&3);

    let v11 : Vec<&i32> = v10.collect();

    println!("{:?}", v11);

    let v9 = vec![10,2,36];

    let v10 = v9.into_iter().filter(|x| x > &3);

    let v11 : Vec<i32> = v10.collect();

    println!("{:?}", v11);

    let mut v9 = vec![10,2,36];

    let v10 = v9.iter_mut().filter(|x| x > &&mut 3);

    let v11 : Vec<& mut i32> = v10.collect();

    println!("{:?}", v11);


}