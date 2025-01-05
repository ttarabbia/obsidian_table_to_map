use std::{
    fs::{self, File},
    io::Read,
};

fn main() -> Result<(), std::io::Error> {
    let dir_path = "./markdown_files/";

    let mut md_contents = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let mut file = File::open(&path)?;

            let mut content = String::new();
            file.read_to_string(&mut content)?;

            md_contents.push(content);

            println!("Read in {:?}", path);
        }
    }
    println!("{:#?}", md_contents);

    Ok(())
}
