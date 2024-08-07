use std::collections::HashMap;


fn main()
{
    let mut scores = HashMap::new();

    scores.insert(String::from("habitat"), 51);
    scores.insert(String::from("finance"), 51);
    let fitness : Option<i32> = scores.insert(String::from("fitness"), 58);

    println!("fitness = {:?}", fitness);
    
    let life_topics = vec![String::from("health"), String::from("familly")];
    let life_score = vec![98, 72];
    let life : HashMap<String,i32> = life_topics.into_iter().zip(life_score.into_iter()).collect();

    println!("life = {:?}", life);

    let nom_champ = "couleur favorite";
    let valeur_champ = "rouge";

    let mut table = HashMap::new();
    table.insert(nom_champ, valeur_champ);

    println!("{}", nom_champ);

    let nom_champ_0 = String::from("couleur favorite");
    let valeur_champ_0 = String::from("rouge");

    let mut table_0 = HashMap::new();
    table_0.insert(nom_champ_0, valeur_champ_0);

    let couleur = table_0.get(&String::from("couleur favorite"));

    match couleur
    {
        Some(qqchose) => println!("{}", qqchose),
        none => println!("y'a pas cette entrée frère"),
    }
    
    let autre_couleur = table_0.get(&String::from("couleur autre"));

    match autre_couleur
    {
        Some(qqchose) => println!("{}", qqchose),
        none => println!("y'a pas cette entrée frère"),
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Bleu"), 10);

    println!("{:?}", scores);

    scores.insert(String::from("Bleu"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Bleu")).or_insert(5554);
    scores.entry(String::from("Rouge")).or_insert(42);

    println!("{:?}", scores);

    let texte = "Il etait une fois dans l'ouest ouest ouest ouest ouest ouest ouest ouest";
    let mut word_count : HashMap<&str, i32> = HashMap::new();

    for word in texte.split_whitespace()
    {
        let compteur = word_count.entry(word).or_insert(0);
        *compteur += 1;
    }

    for (cle, valeur) in &word_count
    {
        println!("{} : {}", cle, valeur);
    }


}



