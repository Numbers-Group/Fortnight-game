use std::fs;
use std::io;

pub fn list_dir(a: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(a)?;

    let file_names: Vec<String> = entries
            .filter_map(|entry| {
                let path = entry.ok()?.path();
                if path.is_file() {
                    path.file_name()?.to_str().map(|s| s.to_owned())
                } else {
                    None
                }
            
            })
            .collect();
    Ok(file_names)
}

pub fn get_projects() {
    // FIXME:
    match list_dir("/Users/twofacedjanus/Documents/geita/geita_projects") {
        Ok(filenames) => {
            for filename in filenames {
                println!("{}", filename);
            }
        }
        Err(e) => println!("Error: {}", e),
    }    
}

