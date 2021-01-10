fn main() {
    loop {
        println!("Hello, world!");
        ::std::thread::sleep(::std::time::Duration::from_secs(2));
    }
}
