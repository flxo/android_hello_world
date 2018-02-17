extern crate android_logger;
#[macro_use]
extern crate log;

fn main() {
    ::android_logger::init_once(log::LogLevel::Trace);
    loop {
        println!("hello world!");
        debug!("hello world!");
        std::thread::sleep(std::time::Duration::new(1, 0));
    }
}
