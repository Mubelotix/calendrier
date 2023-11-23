use calendrier::*;

fn main() {
    let ts = chrono::Utc::now().timestamp();
    let date = DateTime::from_timestamp(Timestamp::from_unix(ts));
    println!("{}", date.to_string_default());
}
