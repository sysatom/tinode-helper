use anyhow::Result;
use delay_timer::prelude::*;
use log::info;

pub async fn setup() -> Result<()>{
    let delay_timer = DelayTimerBuilder::default().build();
    delay_timer.insert_task(build_task_async_store()?)?;
    Ok(())
}

fn build_task_async_store() -> Result<Task, TaskError> {
    let mut task_builder = TaskBuilder::default();

    let body = || async {
        // let store_path = data_dir().unwrap().join("helper").join("store.data");
        // let mut store = StoreBuilder::new(app.clone().app_handle(), store_path).build();
        // store.load().unwrap();
        // let val = store.get("access-url".to_string());
        // let default_val = Value::String("abc".to_string());
        // println!("every 5 sec, {} {}", Local::now(), val.unwrap_or(&default_val));
        info!("scheduler task");
    };

    task_builder
        .set_task_id(1)
        .set_frequency_repeated_by_cron_str("*/5 * * * * *")
        .spawn_async_routine(body)
}