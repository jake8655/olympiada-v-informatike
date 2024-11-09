use std::{cmp::Ordering, env, fs, io};

fn main() {
    let dev_mode = env::args().any(|a| a == "--DEV");

    let input = match dev_mode {
        true => fs::read_to_string("./input").unwrap(),
        false => {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let num_of_lines = input.trim().parse::<i32>().unwrap();
            for _ in 0..num_of_lines {
                let mut line = String::new();
                io::stdin().read_line(&mut line).unwrap();
                input.push_str(&line);
            }
            input
        }
    };

    solve(&input);
}

#[derive(Debug, Default, Clone)]
struct Combination {
    /// The ID of the product
    /// `1 <= product_id <= 200_000`
    product_id: u32,
    /// The price at which the product is being bought
    /// `1 <= buy_price <= 1_000`
    buy_price: u16,
    /// The price at which the product is being sold
    /// `1 <= sell_price <= 1_000`
    sell_price: u16,
    /// The index of the purchase of the product in the list of actions (`input` variable)
    /// `0 <= buy_position <= 200_000`
    buy_position: u32,
    /// The index of the sale of the product in the list of actions (`input` variable)
    /// `0 <= sell_position <= 200_000`
    sell_position: u32,
    /// The profit of the product
    /// ```rs
    /// let profit = sell_price - buy_price
    /// ```
    /// `0 <= profit <= 999`
    profit: u16,
}

impl Combination {
    fn new() -> Self {
        Combination::default()
    }
}

fn solve(input: &str) {
    let lines = input.lines().collect::<Vec<_>>();

    let combinations = form_combinations(lines);
    let filtered_combinations = filter_combinations(combinations);

    let total_profit = filtered_combinations
        .iter()
        .map(|c| c.profit as u32)
        .sum::<u32>();

    println!("{}", total_profit);
}

fn form_combinations(mut lines: Vec<&str>) -> Vec<Combination> {
    // Remove the first line (number of operations)
    lines.remove(0);

    let mut combinations = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        // Skip all sales because we're looking for purchases
        if !line.starts_with("N") {
            continue;
        };

        // We have a buy line
        let parts = line
            .split_whitespace()
            .skip(1)
            .map(|part| {
                part.parse::<u32>()
                    .expect("All parts except the product ID to be of type u32")
            })
            .collect::<Vec<_>>();
        let product_id = parts[0];
        let buy_price = parts[1] as i32;
        let mut combination = Combination::new();

        // Look for a sale with the same product ID
        for (j, sale_line) in lines.iter().skip(i + 1).enumerate() {
            // Skip all sales because we're looking for purchases
            if !sale_line.starts_with("P") {
                continue;
            }

            // We have a sale line
            let parts = sale_line
                .split_whitespace()
                .skip(1)
                .map(|part| {
                    part.parse::<u32>()
                        .expect("All parts except the product ID to be of type u32")
                })
                .collect::<Vec<_>>();
            let sale_product_id = parts[0];
            let sell_price = parts[1] as i32;
            let profit = sell_price - buy_price;

            // The IDs don't match or the profit is less than the combination with the highest profit, so it's a sale that we don't care about at this time
            if sale_product_id != product_id || profit < combination.profit as i32 {
                continue;
            }

            combination.product_id = product_id;
            combination.buy_price = buy_price as u16;
            combination.sell_price = sell_price as u16;
            combination.buy_position = i as u32;
            combination.sell_position = (i + 1 + j) as u32;
            combination.profit = (sell_price - buy_price) as u16;
        }

        // If the product ID is 0, then it's a `default` combination which was only created to
        // compare its profit with the other combinations
        if combination.product_id != 0 {
            combinations.push(combination);
        }
    }

    combinations
}

fn filter_combinations(mut combinations: Vec<Combination>) -> Vec<Combination> {
    for i in 0..combinations.len() {
        if i >= combinations.len() {
            break;
        }

        for j in 0..combinations.len() {
            if j >= combinations.len() {
                break;
            }
            if i == j {
                continue;
            }

            let combination1 = &combinations[i];
            let combination2 = &combinations[j];

            if combination2.buy_position > combination1.buy_position
                && combination2.buy_position < combination1.sell_position
            {
                match combination1.profit.cmp(&combination2.profit) {
                    Ordering::Less => {
                        combinations.remove(i);
                    }
                    Ordering::Greater => {
                        combinations.remove(j);
                    }
                    Ordering::Equal => {
                        combinations.remove(i);
                    }
                }
            } else if combination1.buy_position > combination2.buy_position
                && combination1.buy_position < combination2.sell_position
            {
                match combination2.profit.cmp(&combination1.profit) {
                    Ordering::Less => {
                        combinations.remove(j);
                    }
                    Ordering::Greater => {
                        combinations.remove(i);
                    }
                    Ordering::Equal => {
                        combinations.remove(j);
                    }
                }
            }
        }
    }

    combinations
}
