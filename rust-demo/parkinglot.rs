use std::thread;
use std::sync::{Arc, Mutex};

struct ParkingLot {
    num_spaces: usize,
    spaces_available: Arc<Mutex<usize>>,
}

impl Clone for ParkingLot {
    fn clone(&self) -> Self {
        ParkingLot {
            num_spaces: self.num_spaces,
            spaces_available: self.spaces_available.clone(),
        }
    }
}

impl ParkingLot {
    fn new(num_spaces: usize) -> ParkingLot {
        ParkingLot {
            num_spaces,
            spaces_available: Arc::new(Mutex::new(num_spaces)),
        }
    }

    fn enter_lot(&self) -> bool {
        let mut spaces = self.spaces_available.lock().unwrap();
        if *spaces > 0 {
            *spaces -= 1;
            true
        } else {
            false
        }
    }

    fn exit_lot(&self) {
        let mut spaces = self.spaces_available.lock().unwrap();
        *spaces += 1;
    }
}

fn main() {
    let parking_lot = ParkingLot::new(10);

    for i in 0..20 {
        let parking_lot_clone = parking_lot.clone();
        thread::spawn(move || {
            if parking_lot_clone.enter_lot() {
                println!("Car {} entered the parking lot", i);
                thread::sleep_ms(5000);
                parking_lot_clone.exit_lot();
                println!("Car {} left the parking lot", i);
            } else {
                println!("Car {} couldn't enter the parking lot", i);
            }
        });
    }
}
