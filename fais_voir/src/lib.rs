pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

mod back_office {
    pub mod commande {
        pub fn envoyer_commande (){}
        fn receptionner_commande (){}
    }

    mod administratif {
        fn traiter_avance_ordonnance (){}
        fn traiter_facture_fournisseur (){}
    }
}

mod front_office
{
    pub struct Materiel 
    {
        ordinateur : String,
        pub presentoir : String,
    }

    impl Materiel 
    {
        pub fn creer_materiel (presentoir: &str) -> Materiel
        {
            Materiel
            {
                ordinateur : String::from("Ceci n est pas un ordinateur"),
                presentoir : String::from(presentoir),
            }
        }
    }

    pub mod servir
    {
        pub fn servir_ordonnance (){}
        fn vente_sans_ordonnance (){}
    }

    mod vacciner
    {
        fn faire_vaccin_grippe (){}
        fn faire_vaccin_tiques ()
        {
            super::servir::servir_ordonnance();
        }
    }

}

pub fn travailler_a_la_pharmacie ()
{
    // let m : crate::front_office::Materiel = crate::front_office::Materiel{ordinateur: String::from("mac"), presentoir:String::from("rien")};

    let new_mat = crate::front_office::Materiel::creer_materiel("Vichy");

    crate::front_office::servir::servir_ordonnance();
    back_office::commande::envoyer_commande();
}


