use clap::Parser;
use fs_extra::{
    copy_items,
    dir::CopyOptions,
    file::{read_to_string, write_all},
};
use std::{
    collections::HashMap,
    env::current_dir,
    fs::{create_dir_all, read_dir, remove_file, DirEntry, File},
    path::{Path, PathBuf},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Project name
    project_name: String,

    /// Additional parameters map.
    #[arg(short, long, value_name = "KEY:VALUE")]
    params: Vec<String>,

    /// Output project directory.
    #[arg(value_name = "PATH")]
    output: PathBuf,

    /// Template content path.
    /// Can be directory or ZIP file (must have "zip" extension).
    template: PathBuf,

    /// Print CLI information.
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Cli {
        project_name,
        params,
        output,
        template,
        verbose,
    } = Cli::parse();
    if !output.exists() {
        create_dir_all(&output)?;
    }
    if !template.exists() {
        panic!("* Template does not exists: {:?}", template);
    }
    let params = params
        .into_iter()
        .map(|pair| {
            let mut pair = pair.split(':');
            (
                pair.next().unwrap().to_owned(),
                pair.next().unwrap().to_owned(),
            )
        })
        .chain(std::iter::once((
            "PROJECT_NAME".to_owned(),
            project_name.to_owned(),
        )))
        .collect::<HashMap<_, _>>();
    if verbose {
        println!("* Working directory: {:?}", current_dir()?);
        println!("* Project name: {:?}", project_name);
        println!("* Output: {:?}", output.canonicalize()?);
        println!("* Template: {:?}", output.canonicalize()?);
        println!("* Parameters: {:#?}", params);
    }
    if template
        .extension()
        .map(|ext| ext == "zip")
        .unwrap_or_default()
    {
        zip_extract::extract(File::open(template)?, &output, true)?;
    } else {
        let mut options = CopyOptions::new();
        options.overwrite = true;
        options.copy_inside = true;
        let paths = read_dir(template)?
            .map(|entry| entry.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, _>>()?;
        copy_items(&paths, &output, &options)?;
    }
    visit_dirs(&output, &|entry| {
        let mut path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "chrobry" {
                let content = read_to_string(&path)?;
                remove_file(&path)?;
                path.set_extension("");
                let content = chrobry_core::generate(&content, "", params.to_owned(), |_| {
                    Err("Imports not supported!".to_owned())
                })?;
                write_all(path, &content)?;
            }
        }
        Ok(())
    })?;
    Ok(())
}

fn visit_dirs(
    dir: &Path,
    cb: &dyn Fn(&DirEntry) -> Result<(), Box<dyn std::error::Error>>,
) -> Result<(), Box<dyn std::error::Error>> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}
