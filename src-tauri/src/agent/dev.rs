use log::info;

pub const NAME: &str = "dev";

pub fn instruct(flag: &str) {
    match flag {
        "help_example" => {
            info!("help_instruct...")
        }
        _ => {
            info!("help default instruct");
        }
    }
}