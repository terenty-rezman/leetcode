use std::collections::BTreeMap;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let car_per_hour = 221.0 * (speed as f64);
    let failure_rate = 100.0 - get_success_rate(speed);
    let failed_cars = car_per_hour * failure_rate / 100.0;
    car_per_hour - failed_cars
}

fn get_success_rate(speed: u8) -> f64 {
    let success_rate: BTreeMap<u8, f64> = BTreeMap::from([(1, 100.0), (5, 90.0), (9, 77.0)]);

    for item in success_rate.iter().rev() {
        if speed >= *item.0 {
            return *item.1;
        }
    }
    100.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}

pub fn test() {
    dbg!(production_rate_per_hour(6));
    dbg!(working_items_per_minute(6));
}
