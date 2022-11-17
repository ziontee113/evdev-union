use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct RawInputFragment {
    device_alias: String,
    code: u16,
    pressed_at: SystemTime,
}

impl RawInputFragment {
    pub fn new(device_alias: &str, code: u16, pressed_at: SystemTime) -> Self {
        Self {
            device_alias: device_alias.to_string(),
            code,
            pressed_at,
        }
    }
    pub fn get_device_alias(&self) -> String {
        self.device_alias.to_string()
    }
    pub fn get_code(&self) -> u16 {
        self.code
    }
    pub fn time_since_pressed(&self) -> u128 {
        self.pressed_at.elapsed().unwrap().as_millis()
    }
}
