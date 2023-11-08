// Importaciones necesarias
use std::io;
use serde::{Deserialize, Serialize};

// Función para obtener el precio de una criptomoneda
fn get_crypto(moneda: &str) -> Result<String, ureq::Error> {
    // Realizar una solicitud HTTP para obtener datos de la criptomoneda
    let string = ureq::get(&format!("https://api.coingecko.com/api/v3/coins/{}?localization=false", moneda)).call()?.into_string()?;
    
    // Deserializar los datos JSON en una estructura CoinData
    let coin_data: CoinData = serde_json::from_str(&string).unwrap();
    
    // Devolver el precio en USD como una cadena
    Ok(coin_data.market_data.current_price.usd.to_string())
}

// Definición de estructuras para deserializar datos JSON
#[derive(Serialize, Deserialize, Debug)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    market_data: MarketDate,
}

#[derive(Serialize, Deserialize, Debug)]
struct MarketDate {
    current_price: Prices,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    bmd: f64,
    usd: f64,
}

fn main() {
    let mut coin = String::new();

    // Solicitar al usuario que ingrese la moneda que desea consultar
    println!("¿Ingrese la moneda que desea consultar?");
    
    // Leer la entrada del usuario
    let _ = io::stdin().read_line(&mut coin).expect("Error leyendo la entrada");
    
    // Llamar a la función get_crypto para obtener el precio de la criptomoneda
    let result_precio = get_crypto(&coin);
    
    // Usar el resultado de la función y mostrar el precio o el error
    match result_precio {
        Ok(precio) => println!("El precio es {}", precio),
        Err(error) => println!("Ocurrió un error: {}", error),
    }
}





