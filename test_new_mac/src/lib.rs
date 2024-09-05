#[derive(Debug)]
pub struct aline 
{
    pub l:f32,
}

impl aline {
    pub fn is_bigger(&self, other: &aline) -> bool
    {
        self.l > other.l
    }
    fn affiche_line(&self) -> String
    {
        let r = format!("size = {}", &self.l);
        println!("{}", r);
        r
    }
    fn set_value(&mut self, val : f32)
    {
        if val < 1.0 || val > 100.0
        {
            panic!("T es un ouf toi");
        }
        else {
            self.l = val;
        }
    }
}


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    #[ignore]
    fn mon_test()
    {
        assert_eq!(2,2);
    }

    #[test]
    fn mon_autre_test()
    {
        assert_ne!(3,2);
    }

    #[test]
    fn mon_test_echoue()
    {
        panic!("Echec");
    }

    #[test]
    fn test_contains()
    {
        let myline  = aline{ l : 120.13};
        assert!(myline.affiche_line().contains("2.1"),
        "Le message d'accueil ne contient pas le nom, il vaut {}",
        myline.affiche_line());    
    }

    #[test]
    #[should_panic(expected = "T es un ouf toi")]
    fn test_should_panic ()
    {
        let mut myline = aline {l : 1000.12};
        myline.set_value(0.6);
    }

    #[test]
    fn it_works () -> Result<(), String>
    {
        // Ok(())
        Err(String::from("une string"))
    }

}

