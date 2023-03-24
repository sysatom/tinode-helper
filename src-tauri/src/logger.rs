use env_logger::{Builder, Env};
use std::io::Write;

pub(crate) fn init_logger() {
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info");

    Builder::from_env(env)
        .format(|buf, record| {
            let mut style = buf.style();
            // style.set_bg(Color::Yellow).set_color(Color::White).set_bold(true);

            let timestamp = buf.timestamp();

            writeln!(
                buf,
                "[{}]: {}",
                timestamp,
                style.value(record.args())
            )
        })
        .init();
}