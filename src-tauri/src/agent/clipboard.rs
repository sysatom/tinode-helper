use log::info;

pub const NAME: &str = "clipboard";

pub fn instruct(flag: &str) {
    match flag {
        "clipboard_share" => {
            info!("share clipboard")
        }
        _ => {
            info!("clipboard default instruct");
        }
    }
}
