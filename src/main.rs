use rand::Rng;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const PRINCIPAL_WALLPAPER: &str = "/home/senrique/Wall/principal/";
const SECOND_WALLPAPER: &str = "/home/senrique/Wall/second/";

fn get_file(path: &str) -> String {
    let content = match fs::read_dir(path) {
        Ok(value) => value,
        Err(error) => panic!("não foi possível recuperar o diretório. Err: {}", error),
    };

    let content: Vec<PathBuf> = content
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .collect();

    let index = rand::thread_rng().gen_range(0..=content.len() - 1);

    return content.get(index).unwrap().display().to_string();
}

fn main() {
    let principal_wallpaper = get_file(PRINCIPAL_WALLPAPER);
    let second_wallpaper = get_file(SECOND_WALLPAPER);

    Command::new("feh")
        .arg("--bg-fill")
        .arg(principal_wallpaper)
        .arg(second_wallpaper)
        .output()
        .expect("não foi possível!");
}
