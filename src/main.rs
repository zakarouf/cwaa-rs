//! CWAA-rs - Cat With An Attitude (in rust)
extern crate clap;
extern crate rand;

use clap::{Arg, App};

use std::{
    fs::File,
    io::prelude::*,
    io,
    path::Path,
    fs,
    thread::sleep,
    time::Duration,
};

use rand::{thread_rng, Rng};

struct Aconfig {
    fname: String,
    speed_min: f32,
    speed_max: f32,
    mode :u8,
}

fn file_print_mode_1(aconf: &Aconfig, file_content: &String) {

    let mut rng = thread_rng();
    let mut wait_time = rng.gen_range(aconf.speed_min..aconf.speed_max);

    for ch in file_content.chars() {
        if ch == ' '
        || ch == '\n'
        || ch == '\t' {
            wait_time = rng.gen_range(aconf.speed_min..aconf.speed_max);
            print!("{}", ch);
        } else {
            print!("{}", ch);
            sleep(Duration::from_millis(wait_time as u64));
            io::stdout().flush().unwrap();
        }
    }
}

fn file_print(aconf: &Aconfig, file_path: &Path) {
  
    let file = File::open(&file_path);
    let display = file_path.display();

    let mut file_content = String::new();

    // Evalute Result
    match file {
        Ok(mut f) =>{
            if let Err(why) = f.read_to_string(&mut file_content) {
                println!("Couldn't Read {}: {}", display, why);
            }
        }
        Err(why) => println!("Couldn't Open {}: {}", display, why),
    }  

    match aconf.mode {
        1 => file_print_mode_1(aconf, &file_content),
        _ => println!("{}", file_content),
    }
}

fn cwaa_start(aconf: &Aconfig) {
    // Setting up Path
    let path = Path::new(&aconf.fname);
  
    if path.is_dir() {
        let paths = fs::read_dir(path).unwrap();
        for entry in paths {
            let epath = entry.unwrap().path();
            if epath.is_file() {
                file_print(aconf, epath.as_path());
            }
        }
    } else {
        file_print(aconf, &path);
    }
}

fn main() {
    // Parse Arguments
    let arg_match = App::new("cwaa-rs")
                      .version("0.1.0")
                      .author("Zak D. <zak.d.official@gmail.com>")
                      .about("A Small Text Reader")

                      .arg(Arg::with_name("speed_min")
                          .short("m")
                          .help("Takes in the Minimum Draw speed")
                          .value_name("Float")
                          .takes_value(true)
                          .required(false))

                      .arg(Arg::with_name("speed_max")
                          .short("x")
                          .help("Takes in the Maximum Draw speed")
                          .value_name("Float")
                          .takes_value(true)
                          .required(false))

                      .arg(Arg::with_name("mode")
                          .short("d")
                          .help("Set Draw Mode")
                          .value_name("0 | 1")
                          .takes_value(true)
                          .required(false))
                                      
                      .arg(Arg::with_name("INPUT")
                          .help("Input File/Folder to read")
                          .required(true)
                          .index(1))

                      .get_matches();
    
    let aconf = Aconfig {
        fname: arg_match
                .value_of("INPUT")
                .unwrap()
                .to_string(),

        speed_min: arg_match
                .value_of("speed_min")
                .unwrap_or("10.0")
                .parse::<f32>()
                .unwrap(),

        speed_max: arg_match
                .value_of("speed_max")
                .unwrap_or("50.0")
                .parse::<f32>()
                .unwrap(),
        mode: arg_match
                .value_of("mode")
                .unwrap_or("0")
                .parse::<u8>()
                .unwrap(),
    };


    cwaa_start(&aconf);

}

