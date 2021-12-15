
#[allow(dead_code)]
fn day1(input_data: String) {
    let mut input_numbers: Vec<i32> = Vec::new();

    for number in input_data.split('\n') {
        if let Ok(number) = number.parse::<i32>() {
            input_numbers.push(number);
        }
    }

    let mut last_number = input_numbers[0] + input_numbers[1] + input_numbers[2];
    let mut sliding_window_last = 0;
    let mut greater_counter = 0;

    for x in &input_numbers[3..] {
        let new_window = last_number - input_numbers[sliding_window_last] + x;
        if new_window > last_number {
            greater_counter += 1;
        }
        last_number = new_window;
        sliding_window_last += 1;
    }

    println!("{:?}", greater_counter);
}

#[allow(dead_code)]
fn day2(input_data: String) {
    let mut horizontal_pos = 0;
    let mut aim = 0;
    let mut depth = 0;

    for command in input_data.split('\n') {
        let splitted = command.rsplit_once(' ');
        if splitted.is_some() {
            let (command, value) = splitted.unwrap();
            let value = value.parse::<i32>().unwrap();

            match command {
                "forward" => {
                    horizontal_pos += value;
                    depth += aim * value;
                }
                "up" => aim -= value,
                "down" => aim += value,
                _ => (),
            }
        }
    }
    println!("{:?}", horizontal_pos * depth);
}

#[allow(dead_code)]
fn day3(input_data: String) {

    fn bit_sums(data: &Vec<Vec<u32>>) -> Vec<u32> {

        let mut position_sum: Vec<u32> = Vec::new();

        for binaries in data {

            if position_sum.is_empty() {
                // Assume all binaries have the same length.
                position_sum = vec![0; binaries.len()];
            }

            for i  in 0..binaries.len() {
                position_sum[i] += binaries[i];
            }
        }

        return position_sum;
    }
    
    fn get_most_common_value(data: &Vec<Vec<u32>>) -> Vec<u32> {

        let position_sum: Vec<u32> = bit_sums(data);

        let mut most: Vec<u32> = Vec::new();
        let thresh =  data.len() as f32 / 2.0;

        for x in position_sum {
            if x as f32 >= thresh {
                most.push(1);
            } else {
                most.push(0);
            }
        }

        return most;
    }

    fn get_least_common_value(data: &Vec<Vec<u32>>) -> Vec<u32> {

        let position_sum: Vec<u32> = bit_sums(data);

        let mut least: Vec<u32> = Vec::new();
        let thresh =  data.len() as f32 / 2.0;

        for x in position_sum {
            if x as f32 >= thresh {
                least.push(0);
            } else {
                least.push(1);
            }
        }

        return least;
    }

    let input_data : Vec<Vec<u32>> = input_data.split('\n')
    .filter(|string| !string.is_empty())
    .map(|binaries| binaries.chars().map(|char| char.to_digit(2).unwrap()).collect())
    .collect();

    let gamma = get_most_common_value(&input_data);
    let epsilon = get_least_common_value(&input_data);
    
    let gamma_str : String = gamma.clone().into_iter().map(|b| b.to_string()).collect();

    println!("{:?}", gamma_str);
    let gamma_dec =  u32::from_str_radix(&gamma_str, 2).unwrap();
    println!("{:?}", gamma_dec);

    let epsilon_str : String = epsilon.clone().into_iter().map(|b| b.to_string()).collect();

    println!("{:?}", epsilon_str);
    let epsilon_dec =  u32::from_str_radix(&epsilon_str, 2).unwrap();
    println!("{:?}", epsilon_dec);

    println!("{:?}", epsilon_dec * gamma_dec);

    fn get_rating(data: &Vec<Vec<u32>>, oxygen : bool) -> u32 {

        let mut copied_data : Vec<Vec<u32>> = data.iter().map(|f| f.clone()).collect();

        let mut current_index = 0;
        let mut i : usize = 0;
        let mut common: Vec<u32>;

        if oxygen {
            common = get_most_common_value(&copied_data);
        } else {
            common = get_least_common_value(&copied_data);
        }

        while copied_data.len() > 1 {

            let binaries = &copied_data[i];
            let binary = binaries[current_index];

            if binary != common[current_index]  {
                copied_data.remove(i);
            } else {
                i += 1;
            }

            if i >= copied_data.len() {
                i = 0;
                current_index += 1;
                if oxygen {
                    common = get_most_common_value(&copied_data);
                } else {
                    common = get_least_common_value(&copied_data);
                }
            }
        }

        let rating = &copied_data[0];
        let rating : String = rating.clone().into_iter().map(|b| b.to_string()).collect();
        let rating= u32::from_str_radix(&rating, 2).unwrap();

        return rating;
    }

    let oxygen = get_rating(&input_data, true);
    let co2 = get_rating(&input_data, false);

    println!("oxygen: {:?}, co2: {:?}, result: {:?}", oxygen, co2, oxygen * co2);

}
