use std::thread;
use std::time::{Duration};
use std::thread::JoinHandle;
use chrono::Local;

pub fn time_interval<F>(millsecond: u64, f: F) -> JoinHandle<()>
where
    F: Fn() -> Result<(), ()>,
    F: Sync + 'static,
    F: Send + 'static,
{
    thread::spawn(move || {
        let interval = Duration::from_millis(millsecond);
        //let start = Instant::now();
        //println!("time:{:?}", start);
        loop {
            thread::sleep(interval);
            match f() {
                Ok(()) => {}
                Err(_err) => break,
            };
            //println!("interval time:{:?}", start.elapsed());
        }
    })
}
pub fn get_nanos()->i64{
    let local = Local::now();
    local.timestamp_nanos()
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;
    use std::cell::Cell;

    #[test]
    fn test_time_interval() {}
    #[test]
    fn test_nanos() {
        let mut data=Cell::new(100);
        let t=data.get_mut();

        let p = &data;

        println!("{}",p.get());
        data.set(10);

        p.set(11);
        println!("{}",data.get());

    }
}
