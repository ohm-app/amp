pub mod error;
pub mod manager;

#[cfg(test)]
mod tests {
    use crate::manager::BluetoothManager;
    use btleplug::api::Central;

    #[tokio::test]
    async fn test_bluetooth() {
        let manager = BluetoothManager::new().await.unwrap();
        let adapter_list = &manager.adapters;
        if adapter_list.is_empty() {
            eprintln!("No Bluetooth adapters found");
        }
        for adapter in adapter_list.iter() {
            println!(
                "Starting scan on {}...",
                adapter.adapter_info().await.unwrap()
            );
            let peripherals = manager.scan(None, 10, adapter.clone()).await.unwrap();
            if peripherals.is_empty() {
                eprintln!("->>> BLE peripheral devices were not found, sorry. Exiting...");
            } else {
                println!("Success!");
                dbg!(peripherals);
            }
        }
    }
}
