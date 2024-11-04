pub mod items;
use prost::Message;
use prost_types::Any;

// pub fn create_(amount: i32) -> items::MyStruct {
//     items::MyStruct { val: amount }
// }

pub fn protoc_buf_row_of_db(datas : &items::RowOfDb) ->Vec<u8>
{
    let mut buf = Vec::new();
    datas.encode(&mut buf).expect("Erreur de serialisation");
    buf
}

pub fn deserialize_my_struct(buf: &[u8]) -> Result<items::RowOfDb, prost::DecodeError>
{
    items::RowOfDb::decode(buf)
}

#[cfg(test)]
mod test {
    use items::{TypeSqlx, type_sqlx, NaiveDate, NaiveTime, Void};

    use super::*;

    #[test]
    pub fn test0() {
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


        assert_eq!(one_row[0].types.clone().unwrap(), type_sqlx::Types::Varchar("une string".to_string()));
        assert_eq!(one_row[1].types.clone().unwrap(), type_sqlx::Types::Bigint(19882024));
    }



}

