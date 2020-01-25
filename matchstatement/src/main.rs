
fn match_statment()
{
    let country_code = 90;
    let country = match country_code {
        90 => "Turkey",
        44 => "UK",
        46 => "Sweden",
        1 ..= 999 => "Unknown",
        _ => "Invalid"
    };

    println!("Country = {}",country);
}


fn main() {
    match_statment();
}
