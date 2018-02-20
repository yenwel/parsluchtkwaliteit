extern crate csv;
extern crate serde;
extern crate serde_json;
//extern crate carboxyl;
#[macro_use]
extern crate serde_derive;

use csv::Reader;

#[derive(Debug,Deserialize,Clone)]
struct Record {
    PartitionKey : String,
    RowKey : String,
    Timestamp : String,
    payload : String,
    device : String,
    timePosted : String,
    o3 : String,
    temperature : String,
    humidity : String,
    co : String,
    pressure : String,
    no2 : String,
    batterylevel : String,
    hour : String,
    latitude : String,
    longitude : String,
    minute : String,
    pm1 : String,
    pm10 : String,
    pm25 : String,
    second : String,
    so2 : String,
}

fn main() {
    println!("Running");
    let mut rdr =  
        Reader::from_path("C:/Temp/Data.csv")
            .unwrap();
    let mut records =
        rdr.records()
            .map(|stringrecordresult| 
                stringrecordresult
                    .unwrap()
                    .deserialize::<Record>(None)
                    .unwrap()
                );
    println!("{:?}",records.nth(0).unwrap())
}