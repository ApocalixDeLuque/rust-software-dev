use std::fs::{self};
use std::path::Path;

const DIR_PATH: &str = "/Users/apocalix/Downloads/images";

enum FileTypes {
    Image,
    Document,
    Video,
    Compressed,
    Audio,
    Other
}

impl FileTypes {

    fn matcher (file_name: &str) -> FileTypes {
        let chars = file_name.chars().collect::<Vec<char>>();
        let as_char_slice: &[char] = &chars.as_slice();

        match as_char_slice {
            [.., '.', 'm', 'p', '3'] => FileTypes::Audio,
            [.., '.', 'w', 'a', 'v'] => FileTypes::Audio,
            [.., '.', 'a', 'a', 'c'] => FileTypes::Audio,
            [.., '.', 'a', 'i', 'f', 'f'] => FileTypes::Audio,
            [.., '.', 'w', 'm', 'a'] => FileTypes::Audio,
            [.., '.', 'f', 'l', 'a', 'c'] => FileTypes::Audio,
            [.., '.', 'o', 'g', 'g'] => FileTypes::Audio,
            [.., '.', 'm', 'i', 'd', 'i'] => FileTypes::Audio,
            [.., '.', 'm', '4', 'a'] => FileTypes::Audio,

            [.., '.', 'z', 'i', 'p'] => FileTypes::Compressed,
            [.., '.', 'r', 'a', 'r'] => FileTypes::Compressed,
            [.., '.', '7', 'z'] => FileTypes::Compressed,
            [.., '.', 't', 'a', 'r'] => FileTypes::Compressed,
            [.., '.', 'g', 'z'] => FileTypes::Compressed,
            [.., '.', 'g', 'z', 'i', 'p'] => FileTypes::Compressed,
            [.., '.', 'd', 'm', 'g'] => FileTypes::Compressed,
            [.., '.', 'p', 'k', 'g'] => FileTypes::Compressed,

            [.., '.', 'p', 'd', 'f'] => FileTypes::Document,
            [.., '.', 'd', 'o', 'c'] => FileTypes::Document,
            [.., '.', 'd', 'o', 'c', 'm'] => FileTypes::Document,
            [.., '.', 'd', 'o', 'c', 'x'] => FileTypes::Document,
            [.., '.', 't', 'x', 't'] => FileTypes::Document,
            [.., '.', 'x', 'l', 's'] => FileTypes::Document,
            [.., '.', 'x', 'm', 'l'] => FileTypes::Document,
            [.., '.', 'x', 'l', 's', 'x'] => FileTypes::Document,
            [.., '.', 'p', 'p', 't'] => FileTypes::Document,
            [.., '.', 'p', 'p', 't', 'x'] => FileTypes::Document,
            [.., '.', 'c', 's', 'v'] => FileTypes::Document,
            [.., '.', 'j', 's', 'o', 'n'] => FileTypes::Document,
            [.., '.', 'r', 't', 'f'] => FileTypes::Document,

            [.., '.', 'p', 'n', 'g'] => FileTypes::Image,
            [.., '.', 'j', 'p', 'g'] => FileTypes::Image,
            [.., '.', 'j', 'p', 'e', 'g'] => FileTypes::Image,
            [.., '.', 'g', 'i', 'f'] => FileTypes::Image,
            [.., '.', 'w', 'e', 'b', 'p'] => FileTypes::Image,
            [.., '.', 's', 'v', 'g'] => FileTypes::Image,
            [.., '.', 'i', 'c', 'o'] => FileTypes::Image,

            [.., '.', 'm', 'p', '4'] => FileTypes::Video,
            [.., '.', 'm', '4', 'v'] => FileTypes::Video,
            [.., '.', 'm', 'k', 'v'] => FileTypes::Video,
            [.., '.', 'w', 'e', 'b', 'm'] => FileTypes::Video,
            [.., '.', 'a', 'v', 'i'] => FileTypes::Video,
            [.., '.', 'm', 'o', 'v'] => FileTypes::Video,
            [.., '.', 'm', 'p', 'g'] => FileTypes::Video,
            [.., '.', 'm', 'p', 'e', 'g'] => FileTypes::Video,
            [.., '.', 'f', 'l', 'v'] => FileTypes::Video,
            [.., '.', '3', 'g', 'p'] => FileTypes::Video,
            _ => FileTypes::Other,
        }
    }
}

fn main() -> std::io::Result<()> {
    let dir_path = Path::new(DIR_PATH);

    let image_path = dir_path.join("[v]-Images");
    let doc_path = dir_path.join("[v]-Documents");
    let video_path = dir_path.join("[v]-Videos");
    let compressed_path = dir_path.join("[v]-Compressed");
    let audio_path = dir_path.join("[v]-Audio");

    let paths = [
        &image_path,
        &doc_path,
        &video_path,
        &compressed_path,
        &audio_path,
    ];

    for path in &paths {
        if !path.exists() {
            println!("Path does not exist, creating... {}", &path.display());
            fs::create_dir_all(&path)?;
        }
    }

    let entries = fs::read_dir(dir_path).unwrap();

    for entries in entries {
        let entry = entries.unwrap();
        let path = entry.path();
        let file_path = path.file_name().unwrap();
        let file_name = file_path.to_str().unwrap();

        let file_type = FileTypes::matcher(file_name);

        match file_type {
            FileTypes::Image => {
                fs::rename(dir_path.join(file_name), &image_path.join(file_name))?;
            }
            FileTypes::Document => {
                fs::rename(dir_path.join(file_name), &doc_path.join(file_name))?;
            }
            FileTypes::Video => {
                fs::rename(dir_path.join(file_name), &video_path.join(file_name))?;
            }
            FileTypes::Compressed => {
                fs::rename(dir_path.join(file_name), &compressed_path.join(file_name))?;
            }
            FileTypes::Audio => {
                fs::rename(dir_path.join(file_name), &audio_path.join(file_name))?;
            }
            FileTypes::Other => {}
        }
    }
    
    Ok(())
}
