use std::{fs, io::Error};

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())

    //let text = fs::read_to_string("logs.txt").expect("failed to read logs.txt");
    //let error_logs = extract_errors(text.as_str());
    //fs::write("errors.txt", error_logs.join("\n")).expect("failed to write errors.txt")

    // match divide(5.0, 0.0) {
    //     Ok(result_of_division) => println!("{}", result_of_division),
    //     Err(what_went_wrong) => println!("{}", what_went_wrong),
    // }

    // match validate_email(String::from("asdf")) {
    //     Ok(..) => println!("email is valid"),
    //     Err(error) => println!("{}", error),
    // }

    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let error_logs = extract_errors(text.as_str());

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(()) => println!("Wrote errors.txt"),
    //             Err(error) => println!("Error happened: {}", error),
    //         }
    //     }
    //     Err(error) => println!("Failed to read file: {}", error),
    // }
}

// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//     if b == 0.0 { Err(Error::other("can't divide by 0")) } else { Ok(a / b) }
// }

// fn validate_email(email: String) -> Result<(), Error> {
//     if email.contains("@") { Ok(()) } else { Err(Error::other("emails must have an @")) }
// }
