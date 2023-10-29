use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::Adapter;
use btleplug::Error;

pub struct Manager {
    manager: btleplug::platform::Manager,
    pub adapters: Vec<Adapter>,
}

impl Manager {
    pub async fn new() -> Result<Self, Error> {
        let manager = match btleplug::platform::Manager::new().await {
            Ok(m) => m,
            Err(e) => {
                log::error!("Manager could not be instantiated: {}", e);
                return Err(e);
            }
        };
        let adapters = match manager.adapters().await {
            Ok(a) => a,
            Err(e) => {
                log::error!("Adapters could not be initialized: {}", e);
                return Err(e);
            }
        };
        Ok(Manager { manager, adapters })
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::Manager;
    use btleplug::api::{Central, ScanFilter};
    use tokio::time;

    #[tokio::test]
    async fn test_bluetooth() {
        let manager = Manager::new().await.unwrap();
        let adapter_list = manager.adapters;
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
