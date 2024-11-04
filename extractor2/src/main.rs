use rust_decimal::Decimal;
use std::fmt;
use chrono::NaiveDate;
use chrono::NaiveTime;

pub enum TypeSqlx
{
    BIGDECIMAL(Option<Decimal>),
    STR(Option<String>),
    DATE(Option<chrono::NaiveDate>),
    TIME(Option<chrono::NaiveTime>),
    INT(Option<i64>),
    F64(Option<f64>),
    BOOL(Option<bool>),
    VOID(),
}

impl fmt::Display for TypeSqlx
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            TypeSqlx::BIGDECIMAL(opt) => {
                match opt
                {
                    Some(value) => {write!(f, "{}", value)},
                    None => {write!(f, "{}", "None")}
                }
            },
            TypeSqlx::STR(opt) => {
                match opt
                {
                    Some(value) => {write!(f, "{}", value)},
                    None => {write!(f, "{}", "None")}
                }
            },
            TypeSqlx::DATE(opt) => {
                match opt
                {
                    Some(value) => {write!(f, "{}", value)},
                    None => {write!(f, "{}", "None")}
                }
            },
            TypeSqlx::TIME(opt) => {
                match opt
                {
                    Some(value) => {write!(f, "{}", value)},
                    None => {write!(f, "{}", "None")}
                }
            },
            TypeSqlx::INT(opt) => {
                match opt
                {
                    Some(value) => {write!(f, "{}", value)},
                    None => {write!(f, "{}", "None")}
                }
            },
            TypeSqlx::F64(opt) => {
                match opt
                {
                    Some(value) => {write!(f, "{}", value)},
                    None => {write!(f, "{}", "None")}
                }
            },
            TypeSqlx::BOOL(opt) => {
                match opt
                {
                    Some(value) => {write!(f, "{}", value)},
                    None => {write!(f, "{}", "None")}
                }
            },
            TypeSqlx::VOID() => {write!(f, "{}", "void")},            
        }
    }
}

pub struct RowOfDb
{
    pub cols : Vec<TypeSqlx>,
    pub cols_name : Vec<String>,
    pub size : u32,
}

impl RowOfDb
{
    pub fn new() -> Self
    {
        Self {
            cols : Vec::new(),
            cols_name : Vec::new(),
            size : 0,
        }
    }
}

impl fmt::Display for RowOfDb
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut result : Vec<String> = vec!();

        for e_type_sqlx in self.cols.iter()
        {
            match e_type_sqlx 
            {
                TypeSqlx::BIGDECIMAL(opt) => {
                match opt
                {
                    Some(_) => {result.push(e_type_sqlx.to_string());},
                    None => {result.push("None".to_string());}
                }
                },
                TypeSqlx::STR(opt) => {
                    match opt
                    {
                        Some(_) => {result.push(e_type_sqlx.to_string());},
                        None => {result.push("None".to_string());}
                    }
                },
                TypeSqlx::DATE(opt) => {
                    match opt
                    {
                        Some(_) => {result.push(e_type_sqlx.to_string());},
                        None => {result.push("None".to_string());}
                    }
                },
                TypeSqlx::TIME(opt) => {
                    match opt
                    {
                        Some(_) => {result.push(e_type_sqlx.to_string());},
                        None => {result.push("None".to_string());}
                    }
                },
                TypeSqlx::INT(opt) => {
                    match opt
                    {
                        Some(_) => {result.push(e_type_sqlx.to_string());},
                        None => {result.push("None".to_string());}
                    }
                },
                TypeSqlx::F64(opt) => {
                    match opt
                    {
                        Some(_) => {result.push(e_type_sqlx.to_string());},
                        None => {result.push("None".to_string());}
                    }
                },
                TypeSqlx::BOOL(opt) => {
                    match opt
                    {
                        Some(_) => {result.push(e_type_sqlx.to_string());},
                        None => {result.push("None".to_string());}
                    }
                },
                TypeSqlx::VOID() => {result.push(e_type_sqlx.to_string());},
            }
        }

        let mut result_str = String::new();
        let mut idx = 0;
        for s in result.iter()
        {
            result_str.push_str("   ");
            result_str.push_str(&self.cols_name[idx]);
            result_str.push_str(" : ");

            result_str.push_str(s);
            result_str.push('\n');
            idx += 1;
        }

        write!(f, "size = {}\n[\n{}]", self.size, result_str)

    }
}

use sqlx::{Column, Row, TypeInfo, PgPool};
use sqlx::postgres::{PgPoolOptions, PgRow};

pub async fn create_pg_pool(database_url : &str, max_connect : u32) -> Result<PgPool, sqlx::Error>
{
    PgPoolOptions::new().max_connections(max_connect).connect(database_url).await
}

#[tokio::main]
async fn main()
{

    let t_decimal = TypeSqlx::BIGDECIMAL(Some(Decimal::new(314159, 3)));
    let t_str = TypeSqlx::STR(Some(String::from("wholetsthedogout")));
    let t_date = TypeSqlx::DATE(Some(NaiveDate::from_ymd(1988, 02, 24)));
    let t_time = TypeSqlx::TIME(Some(NaiveTime::from_hms(22,10,0)));
    let t_int = TypeSqlx::INT(None);
    let t_f64 = TypeSqlx::F64(Some(34545.234234));
    let t_bool = TypeSqlx::BOOL(Some(true));
    let t_void = TypeSqlx::VOID();

    let mut row : Vec<TypeSqlx> = Vec::new();

    row.push(t_decimal);
    row.push(t_str);
    row.push(t_date);
    row.push(t_time);
    row.push(t_int);
    row.push(t_f64);
    row.push(t_bool);
    row.push(t_void);


    let mut rowOfDb_0 = RowOfDb::new();

    rowOfDb_0.cols = row;
    rowOfDb_0.cols_name = vec![String::from("decimal"),
                            String::from("string"),
                            String::from("date"),
                            String::from("time"),
                            String::from("int"),
                            String::from("f64"),
                            String::from("bool"),
                            String::from("void")];
    rowOfDb_0.size = 8;

    println!("{}", rowOfDb_0);

    let db = "postgres://postgres:a2020@app_pg:5432/app";
    let pool_of_connections = create_pg_pool(db, 5).await;

    
}




/*
    println!("{}", id_participant);
    let sql_request = "SELECT ae.participation.jour, ae.participation.case FROM ae.participation WHERE participant_id = $1";

    // extraction des données
    let db = "postgres://postgres:a2020@app_pg:5432/app";
    let r_connections = extractor::create_pg_pool(db, 5).await;
    match r_connections
    {
        Ok(connections) =>
        {
            let rows = sqlx::query(&sql_request).bind(id_participant).fetch(&connections);
            // analyse des données
            let mut df = DataFrame::default();
            let r_datas_brut = extract_from_pg_to_df(& mut df, rows).await;
            match r_datas_brut
            {
                Ok(()) =>
                {
                    let r_datas_analyzed = analyse_histogram_present_absent_excuse(&df).await;
                    match  r_datas_analyzed
                    {
                        Ok(datas_analyzed) =>
                        {
                            println!("DATA ANALYZED = {}", datas_analyzed);
                            return convert_df_to_json(datas_analyzed);
                        },
                        Err(_) => 
                        {
                            return String::from("analyse_present_absent_excuse : error")
                        },
                    }
                },
                Err(_) =>
                {
                    return String::from("extract_from_pg_to_df : error")
                },
            }
        },
        Err(_) => 
        {
            return String::from("create_pg_pool : error")
        }
    }


*/