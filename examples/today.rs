use calendrier::*;

fn main() {
    let ts = chrono::Utc::now().timestamp();
    let date = DateTime::from_timestamp(Timestamp::from_unix(ts));
    let name = day_name(date.month(), date.day());
    println!("{} ({name})", date.to_string_default());
}
