#[derive(Debug)]
enum EtatUs {
    Alabama,
    Alaska,
    // -- partie masquée ici --
}

enum PieceUs
{
    Penny,
    Nickel,
    Dime,
    Quarter(EtatUs),
}

fn valeur_en_centimes(piece: PieceUs) -> u8
{
    match piece
    {
        PieceUs::Penny => 
        {
            println!("Il s'agit de la petite piece.");
            1
        },
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        PieceUs::Quarter(etat) =>
        {
            println!("Il s'agit d'un quarter de l'État de {:?} !", etat);
            25
        },
    }
}

fn main()
{
    valeur_en_centimes(PieceUs::Quarter(EtatUs::Alaska));
    valeur_en_centimes(PieceUs::Penny);
}


fn main() {
    let jete_de_de = 9;
    match jete_de_de {
        3 => ajouter_chapeau_fantaisie(),
        7 => enleve_chapeau_fantaisie(),
        autre => deplace_joueur(autre),
    }

    fn ajouter_chapeau_fantaisie() {}
    fn enleve_chapeau_fantaisie() {}
    fn deplace_joueur(nombre_cases: u8) {}
}
