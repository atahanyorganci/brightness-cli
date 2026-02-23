mod display_services;

/// Main display index for DisplayServices API (Apple Silicon / modern macOS).
const MAIN_DISPLAY_ID: i32 = 1;

pub fn get_brightness() -> Result<f32, String> {
    let mut brightness = 0f32;
    let ret =
        unsafe { display_services::DisplayServicesGetBrightness(MAIN_DISPLAY_ID, &mut brightness) };
    if ret == 0 {
        Ok(brightness)
    } else {
        Err("DisplayServicesGetBrightness failed".to_string())
    }
}

pub fn set_brightness(value: f32) -> Result<f32, String> {
    let v = value.clamp(0.0, 1.0);
    let ret = unsafe { display_services::DisplayServicesSetBrightness(MAIN_DISPLAY_ID, v) };
    if ret == 0 {
        Ok(v)
    } else {
        Err("DisplayServicesSetBrightness failed".to_string())
    }
}
