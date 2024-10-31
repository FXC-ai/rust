pub mod items;

pub fn create_my_struct(amount: i32) -> items::MyStruct {
    items::MyStruct { val: amount }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test0() {
        let my_struct0 = create_my_struct(42);
        assert_eq!(my_struct0.val, 42);
    }
}

