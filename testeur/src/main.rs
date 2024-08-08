use std::fs::File;
use std::io;

fn main()
{
    let f = ouvrir_fichier;
    let g = ouvrir_fichier_1;
}

fn ouvrir_fichier() -> Result<File, io::Error>
{
    let f = File::open("hello.txt");

    match f
    {
        Ok(fichier) => Ok(fichier),
        Err(e) => return Err(e),
    }
}

fn ouvrir_fichier_1() -> Result<File, io::Error>
{
    let mut f = File::open("hello.txt")?;
    let ff : Result<File, io::Error> = Ok(f);
    ff
}