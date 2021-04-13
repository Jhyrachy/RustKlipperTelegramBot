//Importo modulo moonrakerAPI
mod moonraker_api;
mod read_settings;
mod structures;

#[tokio::main]
async fn main(){
    //Funzione per ottenere i settaggi dal file settings.config
    let settings_structure = read_settings::read_settings_file().expect("Unable to parse the settings file");

    //Funzione per effettuare la richiesta API dal backend di moonraker
    let moonraker_structure = moonraker_api::moonraker_api_request(settings_structure).await.unwrap();

    println!("Progress: {}", moonraker_structure.result.status.print_stats.state);
}