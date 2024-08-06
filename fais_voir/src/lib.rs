mod back_office;

use crate::back_office::commande;

pub fn gerer_back ()
{
    commande::envoyer_commande();
}