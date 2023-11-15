use std::{
    fs::{self},
    io,
    path::{Path, PathBuf},
    process,
};

use quick_xml::{events::Event, reader::Reader};

const ALLOWED_TYPE: [&str; 3] = ["html", "xhtml", "xml"];

pub fn parse_dir<F: FnMut(String, String)>(dir: &Path, cb: &mut F) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        println!("filepath{path:?}");
        if path.is_dir() {
            parse_dir(&path, cb)?;
        } else if path.is_file() {
            // && ALLOWED_TYPE.contains(&path.extension().unwrap().to_str().unwrap())
            match path.extension() {
                Some(os_str) => match os_str.to_str() {
                    Some(extension) => {
                        if !ALLOWED_TYPE.contains(&extension) {
                            eprintln!("File don't allow: {:?}", os_str);
                            continue;
                        }
                    }
                    None => continue,
                },
                None => continue,
            }
            let (file_name, text) = parse_xml_file(path);
            cb(file_name, text);
        }
    }

    Ok(())
}

fn parse_xml_file(file_path: PathBuf) -> (String, String) {
    // todo!("check file extension");
    let mut reader = Reader::from_file(&file_path).unwrap_or_else(|err| {
        eprintln!("ERROR: could not read file {err}");
        process::exit(1);
    });
    println!("Indexing file: {:?}", file_path);

    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut content = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => eprintln!("Error at position {}: {:?}", reader.buffer_position(), e),

            Ok(Event::Eof) => break,

            Ok(Event::Text(e)) => {
                content.push_str(&e.unescape().unwrap().into_owned());
                content.push('\n');
            }

            _ => (),
        }
        buf.clear();
    }

    (file_path.into_os_string().into_string().unwrap(), content)
}
