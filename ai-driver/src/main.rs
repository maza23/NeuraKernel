use std::fs::{File, OpenOptions, read_to_string};
use std::io::{Write, Read};
use std::thread;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::env;

/// Helper to read an integer from a sysfs file
fn read_int_from_file<P: AsRef<Path>>(path: P) -> Option<u64> {
    std::fs::read_to_string(path).ok()?.trim().parse().ok()
}

fn get_thermal_zone_temp() -> Option<f64> {
    for i in 0..10 {
        let path = format!("/sys/class/thermal/thermal_zone{}/temp", i);
        if let Some(val) = read_int_from_file(&path) {
            return Some(val as f64 / 1000.0);
        }
    }
    None
}

fn get_core_freqs(num_cores: usize) -> Vec<u64> {
    let mut freqs = Vec::new();
    for cpu in 0..num_cores {
        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", cpu);
        if let Some(freq) = read_int_from_file(&path) {
            freqs.push(freq);
        }
    }
    freqs
}

fn set_core_freq(cpu: usize, freq_khz: u64) -> bool {
    let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_setspeed", cpu);
    if let Ok(mut f) = OpenOptions::new().write(true).open(&path) {
        if writeln!(f, "{}", freq_khz).is_ok() {
            return true;
        }
    }
    false
}

fn read_ppt() -> f64 {
    45.0
}

#[derive(Serialize, Deserialize)]
struct SensorData {
    cpu_temp: f64,
    core_freqs: Vec<u64>,
    ppt: f64,
}

fn get_env<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

fn main() {
    dotenvy::dotenv().ok();

    // Load from environment or fallback to defaults
    let sensor_path = env::var("SENSOR_PATH").unwrap_or("/tmp/neurakernel_sensors.json".to_string());
    let action_path = env::var("ACTION_PATH").unwrap_or("/tmp/neurakernel_action.txt".to_string());
    let step_khz = get_env("STEP_KHZ", 200_000u64);
    let min_freq_khz = get_env("MIN_FREQ_KHZ", 800_000u64);
    let max_freq_khz = get_env("MAX_FREQ_KHZ", 4_000_000u64);
    let poll_interval_ms = get_env("POLL_INTERVAL_MS", 100u64);
    let num_cores = get_env("NUM_CORES", 4usize);

    loop {
        let cpu_temp = get_thermal_zone_temp().unwrap_or(0.0);
        let core_freqs = get_core_freqs(num_cores);
        let ppt = read_ppt();

        let data = SensorData {
            cpu_temp,
            core_freqs: core_freqs.clone(),
            ppt,
        };

        let json = serde_json::to_string(&data).unwrap();
        let mut file = File::create(&sensor_path).unwrap();
        file.write_all(json.as_bytes()).unwrap();

        let action = read_to_string(&action_path).unwrap_or("0".to_string());
        let action_val: i64 = action.trim().parse().unwrap_or(0);

        if action_val != 0 && !core_freqs.is_empty() {
            for (i, freq) in core_freqs.iter().enumerate() {
                let mut new_freq = *freq as i64 + action_val * step_khz as i64;
                new_freq = new_freq.clamp(min_freq_khz as i64, max_freq_khz as i64);
                if set_core_freq(i, new_freq as u64) {
                    println!("Set cpu{} to {} kHz", i, new_freq);
                }
            }
        }

        thread::sleep(Duration::from_millis(poll_interval_ms));
    }
}