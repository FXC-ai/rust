fn main()
{
    let a : i32 = 269_000;
    println!("a = {}", a);

    let b : i32 = -0b1111111111111111111111111111111;
    println!("b = {}", b);

    let c : i32 = 0xf;
    println!("c = {}", c);

    let d = 3.14159;
    println!("d = {}", d);

    let f : f64 = 96_485.3321233100184;
    println!("f = {}", f);

    let g : f32 = 96_485.3321233100184;
    println!("g = {}", g);

    let h : bool = true;
    println!("h = {}", h);

    let chat_aux_yeux_de_coeur = '😻';
    println!("chat_aux_yeux_coeur = {}", chat_aux_yeux_de_coeur);

    let mut un_tuple : (i32, f64, u8) = (32, 64.0, 255);
    let trente_deux : i32 = un_tuple.0;
    println!("un_tuple = {}", trente_deux);
    
    un_tuple.0 = 35;
    let (x,y,z) = un_tuple;
    println!("{} {} {}", x,y,z);

    let un_tableau = [299_792_458, 237, 101_325];
    println!("un_tableau = {}", un_tableau[0]);

    let un_tableau1 = [3; 5];
    println!("un_tableau1 = {} {} {} {} {}", un_tableau1[0], un_tableau1[1],un_tableau1[2],un_tableau1[3],un_tableau1[4]);

    let un_tableau2 : [u32; 5] = [1,2,3,4,5];
    println!("un_tableau1 = {} {} {} {} {}", un_tableau2[0], un_tableau2[1],un_tableau2[2],un_tableau2[3],un_tableau2[4]);


}
