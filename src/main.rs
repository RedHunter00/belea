use include_dir::include_dir;
use include_dir::Dir;
use rand::Rng;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::io::{self};
use std::process::Command;
use std::thread;

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

fn open_gif() {
    let file_name;
    match rand::thread_rng().gen_range(1..=6) {
        1 => file_name = "schema_t.gif",
        2 => file_name = "buldozer.gif",
        3 => file_name = "metin2.gif",
        4 => file_name = "skeleton.gif",
        5 => file_name = "truekov.gif",
        6 => file_name = "weak_sperm.gif",
        _ => file_name = "schema_t.gif",
    }

    create_file(file_name);

    #[cfg(target_os = "linux")]
    Command::new("firefox")
        .arg(file_name)
        .output()
        .expect("failed to execute process");

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg("")
        .arg(file_name)
        .output()
        .expect("failed to execute process");
}

fn open_link() {
    let link;
    match rand::thread_rng().gen_range(1..=9) {
        1 => link = "https://www.youtube.com/watch?v=Dx6l-DBoKTI",
        2 => link = "https://youtu.be/3Nkui9-i1DM?t=40",
        3 => link = "https://youtu.be/HfFx5UvzSxc?t=16",
        4 => link = "https://www.youtube.com/watch?v=bGNT5Uh-WKw",
        5 => link = "https://www.youtube.com/watch?v=nU0tyzsrpwI",
        6 => link = "https://www.youtube.com/watch?v=0jChTHzc0Sc",
        7 => link = "https://www.youtube.com/watch?v=yDSNJr__OiQ",
        8 => link = "https://www.youtube.com/watch?v=65EAsE6zrh8",
        9 => link = "https://www.youtube.com/watch?v=EFH9soeufXY",
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

fn play_audio(file_name: &str) {
    create_file(file_name);

    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let decoder = Decoder::new(reader).unwrap();

    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    sink.append(decoder);
    sink.sleep_until_end();
}

fn play_mp3() {
    let file_name;
    match rand::thread_rng().gen_range(1..=9) {
        1 => file_name = "nebunu_weed.mp3",
        2 => file_name = "scoob.mp3",
        3 => file_name = "amogus.mp3",
        4 => file_name = "clash_royale.mp3",
        5 => file_name = "laugh.mp3",
        6 => file_name = "metal_pipe.mp3",
        7 => file_name = "moai.mp3",
        8 => file_name = "mr_beast.mp3",
        9 => file_name = "smecherii.mp3",
        _ => file_name = "nebunu_weed.mp3",
    }

    play_audio(file_name);
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

fn wh() {
    loop {
        println!("F");
    }
}

fn file_bomb() {
    let temp = std::env::temp_dir().to_str().unwrap().to_string();
    let path = temp.clone() + "/oof.txt";
    let path = path.as_str();

    let mut file = OpenOptions::new().append(true).open(path).unwrap();

    for _ in 0..1000000 {
        file.write_all(b"F").ok();
    }

    let source_path = path;
    let mut i = 0;

    loop {
        thread::Builder::new()
            .spawn(|| {
                wh();
            })
            .unwrap();

        let destination_path = temp.clone() + "oof" + i.to_string().as_str() + ".txt";
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
        2 //open gif
    } else if num < 60 {
        3 //link
    } else if num < 90 {
        4 //mp3
    } else if num < 95 {
        5 //vbs unclosable
    } else if num < 98 {
        6 //vbs spam
    } else if num < 99 {
        7 //file, cpu spam
    } else if num < 100 {
        8 //restart
    } else {
        0
    }
}

fn run() {
    match get_option() {
        1 => {
            //nimic
            println!("ai scapat...de data asta :)");
        }
        2 => {
            //open gif
            println!("a picat speciala");
            open_gif();
        }
        3 => {
            //link
            println!("a picat beleua");
            open_link();
        }
        4 => {
            //mp3
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
            thread::Builder::new()
                .spawn(|| {
                    vbs_unclosable();
                })
                .unwrap();
        }
        6 => {
            //vbs spam
            println!("a picat periculoasa");
            thread::Builder::new()
                .spawn(|| {
                    vbs_spam();
                })
                .unwrap();
        }
        7 => {
            //file bomb
            println!("a picat dezastru");
            thread::Builder::new()
                .spawn(|| {
                    file_bomb();
                })
                .unwrap();
        }
        8 => {
            //restart
            println!("a picat nucleara");
            restart();
        }
        _ => (),
    }
}

fn auto(duration: std::time::Duration, audio: bool) {
    if audio {
        loop {
            thread::sleep(duration);

            print!("invarte beleaua dezastrului: Da");
            io::stdout().flush().unwrap();

            thread::Builder::new()
                .spawn(|| {
                    play_audio("slot.mp3");
                })
                .unwrap();
            run();
        }
    } else {
        thread::sleep(duration);

        print!("invarte beleaua dezastrului: ");
        io::stdout().flush().unwrap();
        run();
    }
}

fn pause() {
    print!("invarte beleaua dezastrului: Da");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
}

fn play(audio: bool) {
    if !audio {
        loop {
            pause();
            run();
        }
    } else {
        loop {
            pause();

            thread::Builder::new()
                .spawn(|| {
                    play_audio("slot.mp3");
                })
                .unwrap();
            run();
        }
    }
}

fn main() {
    greet();
    let args: Vec<String> = std::env::args().collect();

    let mut audio = false;
    let mut duration = 0;
    let mut i = 0;
    for arg in args.clone() {
        if arg == "--auto" {
            duration = args[i + 1].parse().unwrap();
        } else if arg == "--audio" {
            audio = true;
        }

        i += 1;
    }

    if audio {
        create_file("slot.mp3");
    }
    if duration != 0 {
        if audio {
            auto(std::time::Duration::from_secs(duration), true);
        } else {
            auto(std::time::Duration::from_secs(duration), false);
        }
    }

    println!("\n\n\n");
    println!("probabilitati:");
    println!("ai scapat: 30%"); //nimic
    println!("speciala: 15%"); //open gif
    println!("belea: 15%"); //link
    println!("necaz: 30%"); //mp3
    println!("obraznica: 5%"); //vbs unclosable
    println!("periculoasa: 5%"); //vbs spam
    println!("dezastru: 1%"); //file, cpu spam
    println!("nucleara: 1%"); //restart

    if audio {
        play(true);
    } else {
        play(false);
    }
}
