use std::env;

fn main() {
    let mut args = env::args();
    let bin_name = args.next().unwrap_or_else(|| "referral_code".to_string());
    let start_arg = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: {} <start_id> <end_id>", bin_name);
            std::process::exit(2);
        }
    };
    let end_arg = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Usage: {} <start_id> <end_id>", bin_name);
            std::process::exit(2);
        }
    };

    let start_id: u32 = match start_arg.parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid start_id: {}", start_arg);
            std::process::exit(2);
        }
    };
    let end_id: u32 = match end_arg.parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid end_id: {}", end_arg);
            std::process::exit(2);
        }
    };

    if start_id > end_id {
        eprintln!("start_id must be <= end_id");
        std::process::exit(2);
    }

    for id in start_id..=end_id {
        match referral::referral_code::map_to_referral_code(id) {
            Ok(code) => println!("{} {}", id, code),
            Err(err) => {
                eprintln!("Error for {}: {:?}", id, err);
                std::process::exit(1);
            }
        }
    }
}
