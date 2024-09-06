use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn simuler_gros_calcul(intensite: u32) -> u32 {
    println!("calcul très lent ...");
    thread::sleep(Duration::from_secs(2));
    intensite
}

struct Cache<T>
where 
    T: Fn(u32) -> u32,
{
    calcul: T,
    valeur:HashMap<u32, u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32
{
    fn new(calcul: T) -> Cache<T>
    {
        Cache {
            calcul,
            valeur: HashMap::new(),
        }
    }

    fn valeur (&mut self, arg: u32) -> u32
    {

        println!("{:?}", self.valeur);

        match self.valeur.get(&arg)
        {
            Some(v) => {*v},
            None => 
            {
                let r = (self.calcul)(arg);
                self.valeur.insert(arg, r);
                r
            }
        }

    }
}

fn generer_exercices(intensite: u32, nombre_aleatoire: u32) 
{
    let mut resultat_lent = Cache::new(|nombre|{
        println!("calcul très lent ...");
        thread::sleep(Duration::from_secs(2));
        nombre
    });

    if intensite < 25 {
        println!(
            "Aujourd'hui, faire {} pompes !",
            resultat_lent.valeur(intensite)
        );
        println!(
            "Ensuite, faire {} abdominaux !",
            resultat_lent.valeur(intensite)
        );
    } else {
        if nombre_aleatoire == 3 {
            println!("Faites une pause aujourd'hui ! Rappelez-vous de bien vous hydrater !");
        } else {
            println!(
                "Aujourd'hui, courrez pendant {} minutes !",
                resultat_lent.valeur(intensite)
            );
        }
    }
}

fn main() {
    let mut valeur_utilisateur_simule = 10;
    let mut nombre_aleatoire_simule = 2;

    generer_exercices(valeur_utilisateur_simule, nombre_aleatoire_simule);


    valeur_utilisateur_simule = 12000;
    nombre_aleatoire_simule = 3;


    generer_exercices(valeur_utilisateur_simule, nombre_aleatoire_simule);

    valeur_utilisateur_simule = 12000;
    nombre_aleatoire_simule = 7;


    generer_exercices(valeur_utilisateur_simule, nombre_aleatoire_simule);

}
