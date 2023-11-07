use btleplug::api::{Central, Manager, ScanFilter};
use btleplug::platform::Adapter;

use crate::error::{AmpError, AmpResult};

#[allow(dead_code)]
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
        tokio::spawn(async move {
            adapter.start_scan(filter.unwrap_or_default()).await?;
            tokio::time::sleep(tokio::time::Duration::from_secs(timeout)).await;
            let peripherals = adapter.peripherals().await?;
            adapter.stop_scan().await.map_err(AmpError::Bluetooth)?;
            Ok(peripherals)
        })
        .await?
    }
}
