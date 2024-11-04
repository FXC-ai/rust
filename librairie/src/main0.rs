use chrono::prelude::*;
use polars_core::prelude::*;
use polars::prelude::*;
use std::fs::File;
use polars::lazy::dsl::{col, pow};
use polars::io::json::{JsonWriter, JsonFormat};
use std::io::Cursor;

fn ecrire_et_lire () -> Result<DataFrame, PolarsError>
{


    let mut my_vector_of_series = Vec::new();
    let mut my_serie = Series::new("a".into(), [String::from("solo")]);
    let mut my_serie1 = Series::new("b".into(), [String::from("han")]);

    
    my_vector_of_series.push(my_serie);
    my_vector_of_series.push(my_serie1);

    let my_df_test = DataFrame::new(my_vector_of_series);
    println!("{}", my_df_test.unwrap());


    let mut my_serie3 = Series::new("b".into(), [Some(String::from("han")), None]);

    println!("{}", my_serie3);
    
    let mut df: DataFrame = df!(
        "name" => ["Alice Archer", "Ben Brown", "Chloe Cooper", "Daniel Donovan"],
        "birthdate" => [
            NaiveDate::from_ymd_opt(1997, 1, 10).unwrap(),
            NaiveDate::from_ymd_opt(1985, 2, 15).unwrap(),
            NaiveDate::from_ymd_opt(1983, 3, 22).unwrap(),
            NaiveDate::from_ymd_opt(1981, 4, 30).unwrap(),
        ],
        "weight" => [63.0, 72.5, 53.6, 83.1],  // (kg)
        "height" => [1.69, 1.77, 1.65, 1.75],  // (m)
    )
    .unwrap();
    println!("AVANT {}", df);

    let result = df.clone().lazy().select([
        col("name"),
        col("birthdate").dt().year().alias("birth_year")])
        .collect()?;
    
    println!("result = {}", df);

    let mut df = df.lazy()
        .with_column(
            (col("weight") / (col("height").pow(2.0))).alias("BMI")
        )
        .collect()?;

    let mut file = File::create("/Users/fx/Documents/tuto-rust/librairie/src/mon_fichier.csv").expect("could not create file");
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df);

    let t1 = CsvReadOptions::default();
    let t2 = t1.with_infer_schema_length(None);
    let t3 = t2.with_has_header(true);
    let t4 = t3.with_parse_options(CsvParseOptions::default().with_try_parse_dates(true));

    let t5 = t4.try_into_reader_with_file_path(Some("/Users/fx/Documents/tuto-rust/librairie/src/mon_fichier.csv".into()))?;
    // Ok(t5)
    let t6 = t5.finish()?;
    println!("t6 {}", t6);


    Ok(t6)

}

// fn main()
// {

//     // ecrire_et_lire();



// }


fn test (mut dafr : & mut DataFrame)
{

    // Create an in-memory buffer
    let mut buffer = Cursor::new(Vec::new());

    // Write the DataFrame to the buffer in JSON format
    let wat = JsonWriter::new(&mut buffer)
        .with_json_format(JsonFormat::Json)
        .finish(& mut dafr);

    // Get the JSON string from the buffer
    let json_str = String::from_utf8(buffer.into_inner()).unwrap();

    // Now `json_str` contains the JSON array of objects
    println!("{}", json_str);
}



fn main()
{

    let mut df = df![
        "id" => &[2, 4, 5, 6, 7, 9, 10, 11, 12, 13],
        "activite_id" => &[7, 8, 90, 90, 90, 89, 14, 89, 89, 890000000],
    ].unwrap();

    test(& mut df);


}


