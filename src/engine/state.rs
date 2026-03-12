use crate::core::config::models::MappingConfig;
use crate::drivers::TabletData;
use std::sync::atomic::AtomicU32;
use std::sync::RwLock;

pub struct SharedState {
    pub config: RwLock<MappingConfig>,
    pub config_version: AtomicU32,
    pub tablet_data: RwLock<TabletData>,
    pub tablet_name: RwLock<String>,
    pub tablet_vid: RwLock<u16>,
    pub tablet_pid: RwLock<u16>,
    pub physical_size: RwLock<(f32, f32)>,
    pub hardware_size: RwLock<(f32, f32)>,
    pub is_first_run: RwLock<bool>,
    pub packet_count: AtomicU32,
    pub stats: RwLock<crate::drivers::DriverStats>,
}
