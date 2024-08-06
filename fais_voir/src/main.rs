use std::{fmt::Result, io::Result as IoResult};

fn fonction1() -> Result {Ok(())}
fn fonction2() -> IoResult<()> {Ok(())}


fn main ()
{
	let r1 = fonction1();
	let r2 = fonction2();
}