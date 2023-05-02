use std::io;

struct ParkingLot {
    num_spaces: usize,
    spaces_used: Vec<String>,
}

impl ParkingLot {
    fn new(num_spaces: usize) -> ParkingLot {

        ParkingLot {
            num_spaces,
            spaces_used: vec!["empty".to_owned(); num_spaces],
        }
    }

}

fn main() {

        println!("Enter a number to initialize Parking Lot Size: ");
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let num: usize = input.trim().parse().expect("Invalid Input");

        let mut parking_lot = ParkingLot::new(num);
        println!("Initialized parking lot of size {}" , num);

    loop {
        println!("Options: ");
        println!("0. Exit Program");
        println!("1. Add Car ");
        println!("2. Remove Car");
        println!("3. Check spot");
        println!("-----------------");
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let num: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number");
                continue;
            }
        };

        if num == 0 {
            println!("Exiting Program");
            break;
        } 
        else if num == 1 {
            println!("Please enter the Parking Spot to enter. (0 - {})", parking_lot.num_spaces -1);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let spot: usize = input.trim().parse().expect("Invalid Input");

            if spot > parking_lot.num_spaces || spot < 0 {
                println!("Spot Out of Range!");
                continue;
            }

            println!("Please enter name/Car/LicensePlate info");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            
            if parking_lot.spaces_used[spot] != "empty" {
                println!("Spot is already Occupied!");
            }
            else {
                parking_lot.spaces_used[spot] = input;
            }


        } 
        else if num == 2 {
            println!("Please enter the Parking Spot to exit. (0 - {})", parking_lot.num_spaces -1);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let spot: usize = input.trim().parse().expect("Invalid Input");

            if spot > parking_lot.num_spaces || spot < 0 {
                println!("Spot Out of Range!");
                continue;
            }
            else {
                parking_lot.spaces_used[spot] = String::from("empty");
            }

        }
        else if num == 3 {
            println!("Please enter the Parking Spot to View. (0 - {})", parking_lot.num_spaces -1);
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let spot: usize = input.trim().parse().expect("Invalid Input");

            if spot > parking_lot.num_spaces || spot < 0 {
                println!("Spot Out of Range!");
                continue;
            }
            else {
                println!("Parking Spot Info: {}", parking_lot.spaces_used[spot]);
            }
        }
        else {
            println!("Invalid input, please enter a number");
            continue;
        }


    }
}