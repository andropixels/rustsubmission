use serde_derive::{Serialize, Deserialize};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write, BufRead};
use std::net::TcpStream;
use std::str::FromStr;
use std::time::Duration;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: simple <mode>");
        std::process::exit(1);
    }

    let mode = &args[1];

    if mode == "--mode=cache" {
        let mut socket = TcpStream::connect("127.0.0.1:8081")?;

        socket.set_read_timeout(Some(Duration::from_secs(10)))?;

        let mut data_points = vec![];

            let price = fetch_btc_price().bitcoin.usd;
            data_points.push(price);
            std::thread::sleep(Duration::from_secs(1));
        

        let aggregate = data_points.iter().sum::<f64>() / data_points.len() as f64;

        let mut file = BufWriter::new(File::create("data.txt")?);
        writeln!(file, "Aggregate: {}", aggregate)?;
        writeln!(file, "Data points:")?;

        for data_point in data_points {
            writeln!(file, "{}", data_point)?;
        }

        println!("Cache complete");
    } else if mode == "--mode=read" {
        let file = File::open("data.txt")?;
        let reader = std::io::BufReader::new(file);

        for line in reader.lines() {
            println!("{}", line?);
        }
    } else {
        eprintln!("Invalid mode");
        std::process::exit(1);
    }

    Ok(())
}



#[derive(Debug, Deserialize, Serialize,Clone)]

struct bitcoin {
usd: f64,

}


#[derive(Debug, Deserialize, Serialize,Clone)]

struct BitcoinPrice {
bitcoin: bitcoin,

}

fn fetch_btc_price() -> BitcoinPrice {
    // this api fetch out the infromation of some cryptocurrencies
    let responce = ureq::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
    .call()
    .unwrap()
    .into_string()
    .unwrap();

    let bitcoin_price: BitcoinPrice = serde_json::from_str(responce.as_str()).unwrap();

    bitcoin_price

}


