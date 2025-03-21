use clap::Parser;
use std::fs;
use std::fs::File;
use std::io::{self, Read, Seek};
use std::process;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(required = true)]
    path: String,

    #[clap(short, long, default_value_t, value_enum)]
    mode: OutputMode,

    #[arg(short, long, default_value = "{path}/{entry},{result}")]
    format: String,
}

#[derive(clap::ValueEnum, Clone, Default, Debug)]
//#[serde(rename_all = "kebab-case")]
enum OutputMode {
    #[default]
    Orientation,
    Resolution,
}

fn output(width: u32, height: u32, mode: &OutputMode) -> io::Result<String> {
    match mode {
        OutputMode::Resolution => Ok(format!("{:?}x{:?}", width, height)),
        OutputMode::Orientation => Ok(if height > width {
            "portrait".to_string()
        } else {
            "landscape".to_string()
        }),
    }
}

fn get_orientation(file_path: &str, output_mode: &OutputMode) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 24];
    file.read_exact(&mut buffer)?;

    if &buffer[0..8] == b"\x89PNG\r\n\x1a\n" {
        let width = u32::from_be_bytes(buffer[16..20].try_into().unwrap());
        let height = u32::from_be_bytes(buffer[20..24].try_into().unwrap());
        return output(width, height, output_mode);
    }

    if buffer[6..10] == *b"JFIF" || buffer[6..10] == *b"Exif" {
        file.seek(io::SeekFrom::Start(2))?;
        let mut marker = [0u8; 2];
        loop {
            file.read_exact(&mut marker)?;
            if marker[0] != 0xFF {
                break;
            }
            if marker[1] >= 0xC0 && marker[1] <= 0xC3 {
                let mut sof = [0u8; 7];
                file.read_exact(&mut sof)?;

                let height = u16::from_be_bytes(sof[3..5].try_into().unwrap());
                let width = u16::from_be_bytes(sof[5..7].try_into().unwrap());

                return output(width as u32, height as u32, output_mode);
            }
            let mut length = [0u8; 2];
            file.read_exact(&mut length)?;
            let skip = u16::from_be_bytes(length) as i64 - 2;
            file.seek(io::SeekFrom::Current(skip))?;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Unsupported file format",
    ))
}

fn main() {
    let args = Args::parse();

    let entries = match fs::read_dir(&args.path) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Error reading directory {}", e);
            process::exit(1);
        }
    };

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() {
                    match get_orientation(path.to_str().unwrap(), &args.mode) {
                        Ok(result) => println!(
                            "{}",
                            args.format
                                .to_string()
                                .replace("{entry}", entry.file_name().to_str().unwrap())
                                .replace("{path}", &args.path)
                                .replace("{result}", &result)
                        ),
                        Err(e) => {
                            eprintln!("Error: {}", e);
                            process::exit(1);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading entry {}", e)
            }
        }
    }
}
