use clap::{Parser, ValueEnum};
use rand::seq::SliceRandom;

use std::{fs, io::Write, process::Command};

fn main() {
    let args = Cli::parse();

    match args.mode {
        Mode::Init => {
            let wallp = read_wallpapers();
            let mon = get_monitors();
            configure_hyprpaper(&wallp, &mon);
        }
        Mode::Reload => {
            eprintln!("Reload is not implemented");
        }
        Mode::Next => {
            let preloaded = get_preloaded();
            let active = get_active();
            let next = select_next(&preloaded, &active);
            set_wallpapers(&next);
        }
    }
}
#[derive(ValueEnum, Clone)]
enum Mode {
    /// Configure the hyprpaper.conf with images from ~/Pictures/hyprland-config-pictures/wallpapers
    Init,
    /// Reload wallpapers from the hyprland-config-pictures directory
    Reload,
    /// Change the wallpaper to the next one in the list of preloaded wallpapers
    Next,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    mode: Mode,
}

fn get_from_hyprpaper(arg: &str) -> Vec<String> {
    let output = Command::new("hyprctl")
        .args(["hyprpaper", arg])
        .output()
        .expect("Failed to execute 'hyprctl hyprpaper listloaded/listactive'");
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_preloaded() -> Vec<String> {
    get_from_hyprpaper("listloaded")
}

fn get_active() -> Vec<String> {
    get_from_hyprpaper("listactive")
}

fn set_to_hyprpaper(wallpapers: &[String], arg: &str) {
    for w in wallpapers {
        Command::new("hyprctl")
            .args(["hyprpaper", arg, w])
            .output()
            .expect("Failed to execute 'hyprctl hyprpaper preload/wallpaper'");
    }
}

fn preload(wallpapers: &[String]) {
    set_to_hyprpaper(wallpapers, "preload");
}

/// `wallpapers` has format `["<monitor>,<path/to/wallpaper>",]`
fn set_wallpapers(wallpapers: &[String]) {
    set_to_hyprpaper(wallpapers, "wallpaper");
}

fn get_monitors() -> Vec<String> {
    let output = Command::new("hyprctl")
        .arg("monitors")
        .output()
        .expect("Failed to execute 'hyprctl monitors'");
    let mut monitors = Vec::with_capacity(1);
    String::from_utf8(output.stdout)
        .unwrap()
        .trim()
        .lines()
        .for_each(|line| {
            if !line.starts_with('\t') {
                let mut line_split = line.split_whitespace();
                line_split.next();
                monitors.push(line_split.next().unwrap().to_string());
            }
        });
    monitors
}

fn read_wallpapers() -> Vec<String> {
    let paths = fs::read_dir(format!(
        "{}/Pictures/hyprland-config-pictures/wallpapers",
        env!("HOME")
    ))
    .expect("Failed to read directory");
    let mut wallpapers = Vec::new();
    paths.for_each(|path| {
        let path = path.unwrap().path();
        if path.is_file()
            && (path.extension().unwrap() == "jpg" || path.extension().unwrap() == "png")
        {
            wallpapers.push(path.to_str().unwrap().to_string());
        }
    });
    wallpapers
}

fn configure_hyprpaper(wallpapers: &[String], monitors: &[String]) {
    let mut file = fs::File::create(format!("{}/.config/hypr/hyprpaper.conf", env!("HOME")))
        .expect("Failed to open hyprpaper.conf");
    file.write_all(b"# This configuration file is automatically generated by wallpaper-handler\n")
        .expect("Failed to write the header to hyprpaper.conf");

    for w in wallpapers {
        file.write_all(format!("preload = {w}\n").as_bytes())
            .expect("Failed to write preloads to hyprpaper.conf");
    }

    let mut randomizer = rand::thread_rng();
    for m in monitors {
        let w = wallpapers.choose(&mut randomizer).unwrap();
        file.write_all(format!("wallpaper = {m},{w}\n").as_bytes())
            .expect("Failed to write wallpapers to hyprpaper.conf");
    }
}
/// `active` has format `["<monitor> = <path/to/wallpaper>",]`
fn select_next(preloaded: &[String], active: &[String]) -> Vec<String> {
    active
        .iter()
        .map(|w| {
            let mut w = w.split(" = ");
            let monitor = w.next().unwrap();
            let wallpaper = w.next().unwrap();
            let next_index =
                (preloaded.iter().position(|x| x == wallpaper).unwrap() + 1) % preloaded.len();
            let next_wallpaper = &preloaded[next_index];

            format!("{monitor},{next_wallpaper}")
        })
        .collect()
}
