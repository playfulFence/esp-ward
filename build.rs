use std::env;

fn main() {
    // Array of all available chip features
    let chip_features = [
        "esp32",
        "esp32s2",
        "esp32s3",
        "esp32c3",
        "esp32c2",
        "esp32c6",
        "esp32h2",
        "esp32-wifi",
        "esp32s2-wifi",
        "esp32s3-wifi",
        "esp32c3-wifi",
        "esp32c2-wifi",
        "esp32c6-wifi",
        "esp32-mqtt",
        "esp32s2-mqtt",
        "esp32s3-mqtt",
        "esp32c3-mqtt",
        "esp32c2-mqtt",
        "esp32c6-mqtt",
    ];

    // This will collect the names of all enabled chip features
    let mut enabled_features = Vec::new();

    for feature in chip_features.iter() {
        if env::var(format!("CARGO_FEATURE_{}", feature.to_uppercase())).is_ok() {
            enabled_features.push(*feature);
        }
    }

    // Ensure exactly one chip feature is enabled
    if enabled_features.len() == 1 {
        println!("cargo:rustc-cfg=feature=\"{}\"", enabled_features[0]);
    } else if enabled_features.is_empty() {
        println!("cargo:warning=You must enable exactly one chip feature (e.g., esp32s2, esp32s3, esp32c3, esp32c6).");
        panic!("No chip feature enabled.");
    } else {
        println!("cargo:warning=You have enabled more than one chip feature: {:?}. You must enable exactly one.", enabled_features);
        panic!("Multiple chip features enabled.");
    }

    if cfg!(feature = "wifi") {
        // If the "wifi" feature is enabled, add additional linker arguments
        println!("cargo:rustc-link-arg=-Trom_functions.x");
    }
}
