use chrono::{Datelike, Local, NaiveTime, Weekday};
use rand::Rng;
use std::{process, io::stdin};
use std::process::Command;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};
use tokio::task;

fn check() {
    let start_time;
    match Local::now().naive_local().weekday() {
        Weekday::Mon => start_time = NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
        Weekday::Tue => start_time = NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
        Weekday::Wed => start_time = NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
        Weekday::Thu => start_time = NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        Weekday::Fri => start_time = NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
        _ => process::exit(0),
    }

    let current_time = Local::now().time();
    let time_diff = (current_time - start_time).num_minutes().abs();

    println!("{}", time_diff);

    if current_time < start_time {
        thread::sleep(Duration::from_secs((60 * time_diff) as u64))
    } else if current_time > start_time && time_diff > 50 {
        process::exit(0)
    }
}

fn get_option() -> i32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=100);
    if num < 25 {
        1
    } else if num < 55 {
        2
    } else if num < 75 {
        3
    } else {
        4
    }
}

async fn process_started(process_names: Vec<&str>) {
    let mut processes = vec![];

    loop {
        let system = System::new_all();
        let process_iter = system.processes();

        for process_name in process_names.as_slice() {
            for (pid, process) in process_iter {
                if process.name().eq_ignore_ascii_case(process_name) {
                    if !processes.contains(pid) {
                        processes.push(pid.clone());
                        nacaz();
                    }
                }
            }
        }

        thread::sleep(Duration::from_secs(1));
    }
}
fn nacaz() {
    let link;
    match get_option() {
        1 => link = "https://www.youtube.com/watch?v=Dx6l-DBoKTI",
        2 => link = "https://youtu.be/3Nkui9-i1DM?t=40",
        3 => link = "https://youtu.be/HfFx5UvzSxc?t=16",
        4 => link = "https://www.youtube.com/watch?v=bGNT5Uh-WKw",
        _ => link = "https://www.youtube.com/watch?v=Dx6l-DBoKTI",
    }
    #[cfg(target_os = "linux")]
    Command::new("firefox")
        .arg(link)
        .output()
        .expect("failed to execute process");

    #[cfg(target_os = "windows")]
    println!("vericuuu!!!");
    let x = "start".to_string() + link;
    Command::new("cmd")
        .args(["/C", x.as_str()])
        .output()
        .expect("failed to execute process");
}

#[tokio::main]
async fn main() {
    //check();

    #[cfg(target_os = "linux")]
    let processes = vec!["konsole", "firefox", "dolphin"];

    #[cfg(target_os = "windows")]
    let processes = vec!["firefox.exe", "chrome.exe", "explorer.exe", "msedge.exe", "codeblocks.exe"];

    let handle = task::spawn(process_started(processes));
    handle.await.unwrap();

    nacaz();
    nacaz();
    nacaz();
    nacaz();

    thread::sleep(Duration::from_secs(1000));
}
