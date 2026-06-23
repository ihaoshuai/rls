use std::{env, fs};

use anyhow::{Ok, Result, anyhow};
use clap::Parser;
use tabled::{Table, settings::{Alignment, Style, object::Columns}};
use rls::{cli, file::FileItem};


fn main() -> Result<()>{
    let args = cli::Arg::parse();
    let files = get_files(&args)?;
    let mut table = Table::new(files);
    table.with(Style::rounded());
    table.modify(Columns::one(1), Alignment::right());

    println!("{}", table);
    
    Ok(())
}

fn get_files(args: &cli::Arg) -> Result<Vec<FileItem>> {
    let mut files = vec![];
    
    let mut target_dir = env::current_dir()?;
    if args.path.is_some() {
        target_dir = target_dir.join(args.path.as_ref().unwrap());
    }

    //check dir path exist
    if !target_dir.exists() {
        return Err(anyhow!("\"{}\" is not exist", target_dir.to_string_lossy()));
    }

    if target_dir.is_file() {
        return Err(anyhow!("\"{}\" is not a dir path", target_dir.to_string_lossy()));
    }

    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        files.push(FileItem::from(&entry)?);
        
    }

    return Ok(files);

}


