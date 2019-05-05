pub fn run() {

    let country_code = 42;

    // Rust match 
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        _ => "unknown"
    };

    println!("The county code with {} is country {}", country_code, country);

    let local_country_code = 1;

    let local_countries = match local_country_code {
        1 => "Zagreb",
        2 => "Bjelovar",
        3 => "Krizevci",
        1...999 => "unknown", // includes 1 to 999 and (999)
        _ => "invalid"
    };

    println!("The local Croatian county code with {} is country {}", local_country_code, local_countries);
}