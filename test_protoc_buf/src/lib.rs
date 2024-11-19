pub mod items;
pub mod polym;
use prost::Message;
use prost_types::Any;

// pub fn create_(amount: i32) -> items::MyStruct {
//     items::MyStruct { val: amount }
// }

// pub fn protoc_buf_row_of_db(datas : &items::RowOfDb) ->Vec<u8>
// {
//     let mut buf = Vec::new();
//     datas.encode(&mut buf).expect("Erreur de serialisation");
//     buf
// }

// pub fn deserialize_my_struct(buf: &[u8]) -> Result<items::RowOfDb, prost::DecodeError>
// {
//     items::RowOfDb::decode(buf)
// }

#[cfg(test)]
mod test {
    


    use polym::VoidStruct;

    use super::*;
    
    #[test]
    pub fn test0() {

        use std::fs::File;
        use std::io::prelude::*;

        use items::{TypeSqlx, type_sqlx, NaiveDate, NaiveTime, Void, RowOfDb};
        let mut one_row = vec!();
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Varchar("une string".to_string()))});
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Bigint(19882024))});
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Numeric(1988.2024))});
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Bool(true))});
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Int(2147483647))});
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Date(NaiveDate{year : 2024, month : 11, day:4}))});
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Time(NaiveTime{hour : 16, minute : 48, second:42}))});
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Varchar("une autre string".to_string()))});
        one_row.push(TypeSqlx {types : Some(type_sqlx::Types::Void(Void{}))});
        one_row.push(TypeSqlx {types : None});

        let row_of_db = RowOfDb {cols : one_row};

        let mut buf = Vec::new();
        row_of_db.encode(&mut buf).expect("Erreur de serialisation");

        let mut r_file = File::create("test0.txt");
        if let Ok(mut file) = r_file
        {
            file.write_all(&buf);
        }

        let c: &[u8] = &buf;
        let row_of_db_decoded = RowOfDb::decode(c).unwrap();


        assert_eq!(row_of_db_decoded.cols[0].types.clone().unwrap(), type_sqlx::Types::Varchar("une string".to_string()));
        assert_eq!(row_of_db_decoded.cols[1].types.clone().unwrap(), type_sqlx::Types::Bigint(19882024));
    }

    #[test]

    pub fn test1(){
        
        use std::fs::File;
        use std::io::prelude::*;

        use polym::{RowOfDb, TypeSqlx, type_sqlx, Int, Varchar, Bigint, Numeric, Date, Time, Void, Bool, NaiveDate, NaiveTime};
        let mut one_row = vec!();
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::S(Varchar{value : "une string".to_string()}))});
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::Bi(Bigint{value : 19882024}))});
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::B(Bool{value : true}))});
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::I(Int{value : 2147483647}))});
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::S(Varchar{value : "une string".to_string()}))});
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::D(Date{value : Some(NaiveDate{year : 2024, month : 11, day:4})}))});
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::T(Time{value : Some(NaiveTime{hour : 16, minute : 48, second:42})}))});
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::S(Varchar{value : "une autre string".to_string()}))});
        one_row.push(TypeSqlx{types : Some(type_sqlx::Types::V(Void{value : Some(VoidStruct{})}))});


        let row_of_db = RowOfDb {cols : one_row};

        let mut buf = Vec::new();
        row_of_db.encode(&mut buf).expect("Erreur de serialisation");

        let mut r_file = File::create("test1.txt");
        if let Ok(mut file) = r_file
        {
            file.write_all(&buf);
        }

        let c: &[u8] = &buf;
        let row_of_db_decoded = RowOfDb::decode(c).unwrap();


        assert_eq!(row_of_db_decoded.cols[0].types.clone().unwrap(), type_sqlx::Types::S(Varchar{value : "une string".to_string()}));
        assert_eq!(row_of_db_decoded.cols[1].types.clone().unwrap(), type_sqlx::Types::Bi(Bigint{value : 19882024}));




    }


}

