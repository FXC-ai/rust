pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// fn main ()
// {
//     println!("{}", add(21,21));



// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
