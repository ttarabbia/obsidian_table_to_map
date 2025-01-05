use std::{
    fs::{self, File},
    io::Read,
};

use markdown::ParseOptions;

fn main() -> Result<(), std::io::Error> {
    let dir_path = "./markdown_files/";

    let mut md_contents: Vec<(String, String)> = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("md") {
            let mut file = File::open(&path)?;

            let mut content = String::new();
            file.read_to_string(&mut content)?;

            let mdast = markdown::to_mdast(&content, &ParseOptions::default());

            mdast.into_iter().map(|n| {match(markdown::Table => println!("{:?}", n); _ => pass)});

            println!("{:?}", mdast);

            md_contents.push((path.display().to_string(), content));

            println!("Read in {:?}", path);
        }
    }
    println!("{:#?}", md_contents);

    Ok(())
}
