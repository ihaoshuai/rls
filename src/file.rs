use std::{fmt::Display, fs::DirEntry, time::SystemTime};

use anyhow::Result;
use tabled::Tabled;


#[derive(Tabled)]
pub struct FileItem {
    name: String,
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
                size: metadata.len(),
                r#type: if metadata.is_dir() { FileType::Dir } else { FileType::File },
                modified: modified_sec,
            })
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