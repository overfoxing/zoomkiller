use std::{thread, time::{Duration, Instant}};
use sysinfo::{Pid, ProcessExt, Signal, System, SystemExt};


fn main() {
    #![windows_subsystem = "windows"]

    let scheduler = thread::spawn(|| {
        let wait_time = Duration::from_secs(2);

        loop {
            let start = Instant::now();

            let _thread_a = thread::spawn(kill_zoom);

            let runtime = start.elapsed();

            if let Some(remaining) = wait_time.checked_sub(runtime) {
                thread::sleep(remaining);
            }
        }
    });

    scheduler.join().expect("Scheduler panicked");
}

fn kill_zoom() {
    let s = System::new_all();

    for process in s.processes_by_name("Zoom") { // Find all Zoom processes
        println!("{} {}", process.pid(), process.name());
        
        if let Some(process) = s.process(Pid::from(process.pid())) {
            if process.kill_with(Signal::Kill).is_none() {
                println!("This signal isn't supported on this platform");
            }
        }
    }

    thread::sleep(Duration::from_millis(100));
}