use chrono::Utc;
use date_printer::print_date;
fn main() {
    print_date(&Utc::now());
}
