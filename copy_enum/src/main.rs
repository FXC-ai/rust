
fn main ()
{
	let une_valeur_u8 : Option<u8> = Some(3);

	match une_valeur_u8 {
		Some(max) => println!("{}", max),
		_ => (),
	}
	if let Some(qqchose) = une_valeur_u8 {
		println!("{}", qqchose);
	}
	else
	{
		()
	}
}