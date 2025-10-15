fn main() {
    for _ in 0..num_cpus::get() {
        std::thread::spawn(|| loop {});
    }

    loop {}
}
