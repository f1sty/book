use std::path::Path;
use std::process::Command;

fn main() {
    let sound_path = Path::new("/home/f1sty/downloads/ding.mp3");
    if let Ok(_child) = Command::new("mpv").arg(sound_path).spawn() {
        println!("Ding!");
    } else {
        println!("Couldn't run command");
    }
}
