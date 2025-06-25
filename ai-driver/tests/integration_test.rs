#[test]
fn test_dummy_sensor_data() {
    use ai_driver::SensorData;
    let data = SensorData {
        cpu_temp: 55.0,
        core_freqs: vec![3000, 3100, 3200, 3300],
        ppt: 42.0,
    };
    assert_eq!(data.core_freqs.len(), 4);
    assert!(data.cpu_temp >= 0.0);
}