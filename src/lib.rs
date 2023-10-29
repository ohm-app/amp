#[cfg(test)]
mod tests {
    use std::time::Duration;

    use btleplug::api::{Central, Manager as _, ScanFilter};
    use btleplug::platform::Manager;
    use tokio::time;

    #[tokio::test]
    async fn test_bluetooth() {
        let manager = Manager::new().await.unwrap();
        let adapter_list = manager.adapters().await.unwrap();
        if adapter_list.is_empty() {
            eprintln!("No Bluetooth adapters found");
        }
        for adapter in adapter_list.iter() {
            println!(
                "Starting scan on {}...",
                adapter.adapter_info().await.unwrap()
            );
            adapter
                .start_scan(ScanFilter::default())
                .await
                .expect("Can't scan BLE adapter for connected devices...");
            time::sleep(Duration::from_secs(10)).await;
            let peripherals = adapter.peripherals().await.unwrap();
            if peripherals.is_empty() {
                eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
            } else {
                println!("Success!");
                dbg!(peripherals);
            }
        }
    }
}
