fn main() {
    let current_temp: f64 = 16.5;

    if current_temp < 18.0 {
        println!("Thermostat: HEATER ON");
    }
    else if current_temp <= 25.0 {
        println!("Thermostat: IDLE");
    } else {
        println!("Thermostat: AC ON");
    }
}