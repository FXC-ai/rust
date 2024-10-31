pub mod test_protoc_buf {
    pub mod items {
        include!(concat!(env!("OUT_DIR"), "/test_protoc_buf.items.rs"));
    }
}

use crate::test_protoc_buf::items;
use items::MyStruct;

pub fn create_my_struct(amount : i32 ) -> items::MyStruct 
{
    MyStruct {val : amount}
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    pub fn test0()
    {
        let my_struct0 = create_my_struct(42);
        assert_eq!(my_struct0.val, 42);
    }
}
