pub mod error;

use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::Adapter;
use error::{AmpError, AmpResult};

pub struct BluetoothManager {
    manager: btleplug::platform::Manager,
    pub adapters: Vec<Adapter>,
}

impl BluetoothManager {
    pub async fn new() -> AmpResult<Self> {
        log::trace!("Manager: Creating btleplug::platform::Manager...");
        let manager = match btleplug::platform::Manager::new().await {
            Ok(m) => m,
            Err(e) => {
                log::error!("Manager could not be instantiated: {}", e);
                return Err(AmpError::Bluetooth(e));
            }
        };
        log::trace!("Instantialted bleplug::platform::Manager: {:?}", manager);
        let adapters = match manager.adapters().await {
            Ok(a) => a,
            Err(e) => {
                log::error!("Adapters could not be initialized: {}", e);
                return Err(AmpError::Bluetooth(e));
            }
        };
        log::trace!("Adapters: {:?}", adapters);
        Ok(BluetoothManager { manager, adapters })
    }

    pub async fn scan(
        &self,
        filter: Option<ScanFilter>,
        timeout: u64,
        adapter: Adapter,
    ) -> AmpResult<Vec<btleplug::platform::Peripheral>> {
        log::trace!(
            "Starting scan on {}...",
            adapter.adapter_info().await.unwrap()
        );
        match tokio::spawn(async move {
            adapter
                .start_scan(filter.unwrap_or_default())
                .await
                .expect("Can't scan BLE adapter for connected devices...");
            tokio::time::sleep(tokio::time::Duration::from_secs(timeout)).await;
            let peripherals = adapter
                .peripherals()
                .await
                .expect("Failed to get peripherals.");
            adapter
                .stop_scan()
                .await
                .expect("An error occurred while stopping the scan.");
            peripherals
        })
        .await
        {
            Ok(o) => Ok(o),
            Err(e) => Err(AmpError::Thread(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::BluetoothManager;
    use btleplug::api::{Central, ScanFilter};
    use tokio::time;

    #[tokio::test]
    async fn test_bluetooth() {
        let manager = BluetoothManager::new().await.unwrap();
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
