fn main() {

    println!("start db\n");
    _ =db(); 
    println!("\n");
    println!("completed operation\n");
}
use serde_derive::{Serialize, Deserialize};
use mysql::*;
use mysql::prelude::*;


#[derive(Debug, Deserialize, Serialize,Clone)]
struct CoinsbasePrices {
data: Vec<CoinsPrices>,

} 

#[derive(Debug, Deserialize, Serialize,Clone)]

struct CoinsPrices {
id: String,
name: String,
min_size: String,
}

///db operations
fn db() -> std::result::Result<(), Box<dyn std::error::Error>>  {
    println!("getting in");
    //here you can change user and database according to your need
    let url = "mysql://root:password@localhost:3306/mysql";
    let pool = Pool::new(url)?;

    let mut conn = pool.get_conn()?;
    println!("connection has been made\n");

    conn.query_drop(
    r"CREATE TABLE IF NOT EXISTS ctest (
        id text not null,
        name text not null,
        min_size text
    )")?;

    println!("table has been created\n");

    let responce  = fetch_responce(); 

    let coin_base_price: CoinsbasePrices = serde_json::from_str(responce.as_str()).unwrap();

    println!("responce has been fetched\n");
    
    conn.exec_batch(
        r"INSERT INTO ctest (id, name, min_size)
        VALUES (:id, :name, :min_size)",
        coin_base_price.data.iter().map(|p| params! {
            "id" => &p.id,
            "name" => &p.name,
            "min_size" => &p.min_size,
        })
    )?;
    println!("data inserted into tables\n");

    let selected_payments = conn
    .query_map(
        "SELECT * from ctest",
        |(id, name, min_size)| {
            CoinsPrices { id, name, min_size}
        },
    )?;

    print!("\n");
    print!("\n");
    print!("\n");

    for coin in selected_payments {

        println!("id: {}",coin.id);
        println!("name: {}",coin.name);
        println!("min_size: {}",coin.min_size);
        print!("\n");
        print!("\n");
    }

    Ok(())
}


 ///fetch responce from the api 
 fn fetch_responce() -> String {
    // this api fetch out the infromation of some cryptocurrencies
    let responce = ureq::get("https://api.coinbase.com/v2/currencies")
    .call()
    .unwrap()
    .into_string()
    .unwrap();

    responce

}
