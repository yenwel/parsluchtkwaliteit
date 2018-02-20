extern crate csv;
extern crate serde;
extern crate serde_json;
//extern crate carboxyl;
#[macro_use]
extern crate serde_derive;
extern crate base64;

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

#[derive(Debug,Deserialize,Clone)]
struct Payload {
    devAddr : String,
    packetIdentifier : String,
    packetsLeft : f64,
    gatewayEui : String,
    packetTime : String,
    localTime : String,
    tmst : f64,
    frequency : f64,
    dataRate : String,
    rssi : f64,
    snr : f64,
    fcnt : String,
    micValid : String,
    payload : String,
    rawData : String,
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
            )
            .map(|record| 
                    (
                        record.clone(),
                        serde_json::from_str::<Payload>(&record.payload.clone()).unwrap()
                    )
            )
            .map(
                |tuple|
                (
                    tuple.0.clone(),
                    tuple.1.clone(),
                    String::from_utf8(base64::decode(&tuple.1.payload).unwrap())
                )
            );
    println!("{:?}",records.nth(0).unwrap())
}