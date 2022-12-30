use std::io;

fn main() {
    println!("Cook a Brilian lasagna!");
    println!(
        "Here are your options: 
    \n 1. Check the expected minutes in the oven. 
    \n 2. If it's already cooking calculate the remaining oven time in minutes 
    \n 3. Calculate the prep time. 
    \n 4. Calculate the elapsed time; Enter your option below: "
    );

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Could not read line");

    let option: u8 = option.trim().parse().expect("Expected a number.");

    if option == 1 {
        println!(
            "The expected oven time in minutes is: {}",
            expected_minutes_in_oven()
        );
    } else if option == 2 {
        println!("How many minutes has it been in the oven?");

        let mut remaining_minutes = String::new();

        io::stdin()
            .read_line(&mut remaining_minutes)
            .expect("Could not read line");

        let remaining_minutes: u8 = remaining_minutes
            .trim()
            .parse()
            .expect("Expected an integer.");

        println!(
            "The remaining minutes in oven is: {}",
            remaining_minutes_in_oven(remaining_minutes)
        )
    } else if option == 3 {
        println!("How many layers are you planning to cook?");

        let mut layer_count = String::new();

        io::stdin()
            .read_line(&mut layer_count)
            .expect("Could not read line");

        let layer_count: u8 = layer_count.trim().parse().expect("Expected an integer.");

        println!(
            "Preparation time in minutes is: {}",
            preparation_time_in_minutes(layer_count)
        )
    } else if option == 4 {
        println!("How many layers are you planning to cook?");

        let mut layer_count = String::new();

        io::stdin()
            .read_line(&mut layer_count)
            .expect("Could not read line");

        let layer_count: u8 = layer_count.trim().parse().expect("Expected an integer.");

        println!("How many minutes has it been in the oven?");

        let mut oven_time = String::new();

        io::stdin()
            .read_line(&mut oven_time)
            .expect("Could not read line");

        let oven_time: u8 = oven_time.trim().parse().expect("Expected an integer.");

        println!(
            "The time elapsed is: {}",
            elapsed_time_in_minutes(layer_count, oven_time)
        );
    }
}

fn expected_minutes_in_oven() -> u8 {
    const EXPECTED_OVEN_TIME: u8 = 40;
    return EXPECTED_OVEN_TIME;
}

fn remaining_minutes_in_oven(entry: u8) -> u8 {
    let remain = { expected_minutes_in_oven() - entry };
    return remain;
}

fn preparation_time_in_minutes(layers: u8) -> u8 {
    const MINUTES_FOR_EACH_LAYER: u8 = 2;
    let duration = { MINUTES_FOR_EACH_LAYER * layers };
    return duration;
}

fn elapsed_time_in_minutes(num_layers: u8, oven_munites: u8) -> u8 {
    let time_lapse = { preparation_time_in_minutes(num_layers) + oven_munites };

    return time_lapse;
}
