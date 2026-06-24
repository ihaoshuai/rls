use std::{fmt::Display, fs::DirEntry, path::PathBuf, time::SystemTime};

use anyhow::Result;
use tabled::Tabled;

#[cfg(unix)]
pub use unix::*;
#[cfg(windows)]
pub use windows::*;

#[derive(Tabled)]
pub struct FileItem {
    name: String,
    #[tabled(skip)]
    path: PathBuf,
    #[tabled(display("file_size_format"))]
    size: u64,
    #[tabled(rename = "type")]
    r#type: FileType,
    #[tabled(display("modified_sec_format"))]
    modified: u64,
}


impl FileItem {
    pub fn from(entry: &DirEntry) -> Result<Self> {
        let metadata = entry.metadata()?;
        let file_name = entry.file_name().to_string_lossy().into_owned();
        let modified_sec = SystemTime::now().duration_since(metadata.modified()?)?.as_secs();
        Ok(Self {
                name: file_name,
                path: entry.path(),
                size: metadata.len(),
                r#type: if metadata.is_dir() { FileType::Dir } else { FileType::File },
                modified: modified_sec,
            })
    }

    pub fn du(&mut self) -> Result<()>
    {
        if self.r#type == FileType::Dir {
            self.size = get_dir_size(&self.path)?;
        }
        Ok(())
    }
}

fn file_size_format(size: &u64) -> String {
    let mut size = *size;
    let size_units = [ "B", "KB", "MB", "GB", "TB" ];
    let mut i = 0;
    while size > 1024 && i < size_units.len()-1 {
        size /= 1024;
        i += 1;
    }
    return format!("{} {}", size, size_units[i]);
}

fn modified_sec_format(sec: &u64) -> String {
    let divs = [ 0, 24, 60, 60 ];
    let time_unit = [ "days", "hours", "minutes", "seconds" ];
    let mut res = [ 0, 0, 0, *sec ];
    let mut i = 3;
    while i > 0  {
        res[i-1] = res[i] / divs[i];
        if res[i-1] == 0 {
            break;
        }
        i -= 1;
    };
    return format!("{} {} ago", res[i], time_unit[i]);
    
}

#[derive(PartialEq, Eq)]
enum FileType {
    Dir,
    File,
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output_str = match self {
            FileType::Dir => "dir",
            FileType::File => "file",
        };
        write!(f, "{}", output_str)?;
        Ok(())
    }
}


#[cfg(unix)]
mod unix {
    use std::{fs, path::Path};

    use anyhow::{Ok, Result};

    // TODO why the blocks() method alawys return zero? it is not work
    // pub fn get_dir_size(metadata: &Metadata) -> u64
    // {
    //     use std::os::unix::fs::MetadataExt;
    //     println!("{}", metadata.blocks() * 512);
    //     metadata.blocks() * 512
    // }


    pub fn get_dir_size<P: AsRef<Path>>(path: P) -> Result<u64>
    {
        let mut size = 0;
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let meta = entry.metadata()?;
            if meta.is_dir() {
                size += get_dir_size(entry.path())?;
            }else {
                size += meta.len();
            }
        }
        Ok(size)
    }
}


#[cfg(windows)]
mod windows {
    use std::path::Path;
    use anyhow::Result;

    pub fn get_dir_size<P: AsRef<Path>>(path: P) -> Result<u64>
    {
        // TODO windows platform to get dir size
        Ok(0)
    }
}