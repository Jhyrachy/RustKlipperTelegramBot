use crate::structures::MoonrakeStructure;
use hyper::Client;
use hyper::body;
use crate::structures::SettingsStruct;



pub async fn moonraker_api_request(settings: SettingsStruct) -> hyper::Result<MoonrakeStructure> {
    let client = Client::new();

    //Creazione URL con i parametri, uso i riferimenti alla struttura.
    //usare .parse()? per ottenere dinamicamente il contenuto in tipo URI per api_request_url
    let api_request_url = format!("http://{}:{}/printer/objects/query?{}",
                                    settings.host_system,
                                    settings.moonraker_port,
                                    settings.moonraker_arguments)
                                    .parse()
                                    .unwrap();

    //Mette la risposta dell'url nella variabile 'moonraker_response', .await è l'attesa asincrona.
    //'mut'consente di cambiare la variabile
    let moonraker_response = client.get(api_request_url)
                                            .await?;

    //Studia lo stato della risposta, positivo è 200
    //println!("Response: {}", moonraker_response.status());

    let request_bytes = body::to_bytes(moonraker_response.into_body()).await?;
    let moonrake_answer = String::from_utf8(request_bytes.to_vec()).expect("response was not valid utf-8");
    //Output del contenuto della chiamata
    //println!("Answer: {}", moonrake_answer);

    //Parsing da stringa in json
    let moonrake_structure: MoonrakeStructure = serde_json::from_str(&moonrake_answer).expect("Unable to parse .json file");

    Ok(moonrake_structure)
}