use chrono::Local;
use chrono::DateTime;

struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::new()
    }

    fn set() -> ! {
        unimplemented!()
    }
}

fn main() {
    let now = Local::now();
    println!("{}", now);
}
