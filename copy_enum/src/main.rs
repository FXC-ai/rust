#[derive(Debug)]
enum Trois
{
	EnNombre(i32),
	EnStr(String),
	Autre,
}

fn plus_un (x : Option<i32>) -> Option<i32>
{
	match x
	{
		None => None,
		Some(p) => Some(p+1),
	}
}


fn main ()
{
	let cinq = plus_un(Some(5));
	println!("{:?}", cinq);
	let nul = plus_un(None);
	println!("{:?}", nul);


	println!("t {:?}", Trois::EnNombre(42));

	let var_test = "trois en nombre";

	let result_match = match var_test
	{
		"trois en nombre" => Trois::EnNombre(3),
		"trois en chiffre" => Trois::EnStr(String::from("trois")),
		_ => Trois::Autre,
	};


	println!("{:?}", result_match);

}