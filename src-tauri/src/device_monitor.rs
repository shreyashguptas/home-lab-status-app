use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub ip: String,
    pub status: bool,
}

impl Device {
    pub fn new(name: String, ip: String) -> Self {
        Self {
            name,
            ip,
            status: false,
        }
    }

    pub async fn check_status(&mut self) {
        let result = ping_rs::ping(&self.ip, Duration::from_secs(1), None).await;
        self.status = result.is_ok();
    }
}

pub struct DeviceMonitor {
    devices: Vec<Device>,
}

impl DeviceMonitor {
    pub fn new() -> Self {
        Self { devices: Vec::new() }
    }

    pub fn add_device(&mut self, name: String, ip: String) {
        self.devices.push(Device::new(name, ip));
    }

    pub fn remove_device(&mut self, index: usize) {
        if index < self.devices.len() {
            self.devices.remove(index);
        }
    }

    pub fn get_devices(&self) -> &[Device] {
        &self.devices
    }

    pub async fn check_all_devices(&mut self) {
        for device in &mut self.devices {
            device.check_status().await;
        }
    }

    pub async fn start_monitoring(&mut self) {
        loop {
            self.check_all_devices().await;
            sleep(Duration::from_secs(60)).await;
        }
    }
} 