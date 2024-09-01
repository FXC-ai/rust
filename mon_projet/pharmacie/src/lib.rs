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
    mod commande {
        fn envoyer_commande (){}
        fn receptionner_commande (){}
    }

    mod administratif {
        fn traiter_avance_ordonnance (){}
        fn traiter_facture_fournisseur (){}
    }
}

mod front_office {
    mod servir {
        fn servir_ordonnance (){}
        fn vente_sans_ordonnance (){}
    }

    mod vacciner {
        fn faire_vaccin_grippe (){}
        fn faire_vaccin_tiques (){}
    }
}

pub fn travailler_a_la_pharmacie ()
{
    crate::front_office::servir::servir_ordonnance();
    back_office::commande::envoyer_commande();


}