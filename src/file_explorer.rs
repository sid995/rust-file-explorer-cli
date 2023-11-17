use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;
use std::num::TryFromIntError;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct FileExplorer {
    pub current_path: PathBuf,
}

impl FileExplorer {
    pub fn new() -> Self {
        FileExplorer {
            current_path: env::current_dir().unwrap(),
        }
    }

    pub fn get_current_directory(&self) -> &Path {
        &self.current_path
    }

    pub fn list_directory_with_properties(&self) -> io::Result<()> {
        println!("{:<25} {:<15} {}", "File/Directory", "Size (bytes)", "Last Modified");
        println!("{}", "------------------------------------------------------------");

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

        for entry in fs::read_dir(&self.current_path)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_path = entry.path();
            let metadata = fs::metadata(&file_path)?;
            let file_size = if metadata.is_file() {
                metadata.len().to_string()
            } else {
                "<DIR>".to_string()
            };

            // Calculate last modified time relative to the current time
            let last_modified = metadata.modified()?.duration_since(UNIX_EPOCH);
            let relative_last_modified = match last_modified {
                Ok(duration) => current_time.checked_sub(duration).unwrap_or_else(|| Duration::from_secs(0)),
                Err(_) => Duration::from_secs(0),
            };
            println!(
                "{:<25} {:<15} {} seconds ago",
                file_name.to_string_lossy(),
                file_size,
                relative_last_modified.as_secs()
            );
        }

        Ok(())
    }

    pub fn copy_file(&self, source: &Path, destination: &Path) -> io::Result<()> {
        let mut source_file = File::open(source)?;
        let mut destination_file = File::create(destination)?;

        io::copy(&mut source_file, &mut destination_file)?;

        println!("File copied successfully.");

        Ok(())
    }

    pub fn delete_file(&self, file_path: &Path) -> io::Result<()> {
        fs::remove_file(file_path)?;

        println!("File deleted successfully.");

        Ok(())
    }

    pub fn create_file(&self, file_path: &Path) -> io::Result<()> {
        File::create(file_path)?;

        println!("File created successfully.");

        Ok(())
    }

    pub fn navigate_up(&mut self) {
        if let Some(parent_path) = self.current_path.parent() {
            self.current_path = parent_path.to_path_buf();
            println!("Navigated up to: {:?}", self.current_path);
        } else {
            println!("Already at the root directory.");
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        loop {
            println!("Current Directory: {:?}", self.current_path);
            println!("{}", "-----------------------------");
            self.list_directory_with_properties()?;
            println!("{}", "-----------------------------");
            println!("Options:");
            println!("  1. Change Directory");
            println!("  2. Copy File");
            println!("  3. Delete File");
            println!("  4. Create New File");
            println!("  5. Navigate Up");
            println!("  6. List Directory with Properties");
            println!("  7. Exit");

            print!("Enter choice: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let choice: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid choice. Try again.");
                    continue;
                }
            };

            match choice {
                1 => {
                    print!("Enter directory name: ");
                    io::stdout().flush()?;
                    let mut new_dir = String::new();
                    io::stdin().read_line(&mut new_dir)?;
                    new_dir = new_dir.trim().to_string();
                    self.current_path.push(new_dir);
                }
                2 => {
                    print!("Enter source file: ");
                    io::stdout().flush()?;
                    let mut source_file = String::new();
                    io::stdin().read_line(&mut source_file)?;

                    print!("Enter destination file: ");
                    io::stdout().flush()?;
                    let mut destination_file = String::new();
                    io::stdin().read_line(&mut destination_file)?;

                    self.copy_file(
                        &self.current_path.join(source_file.trim()),
                        &self.current_path.join(destination_file.trim()),
                    )?;
                }
                3 => {
                    print!("Enter file to delete: ");
                    io::stdout().flush()?;
                    let mut file_to_delete = String::new();
                    io::stdin().read_line(&mut file_to_delete)?;

                    self.delete_file(&self.current_path.join(file_to_delete.trim()))?;
                }
                4 => {
                    print!("Enter new file name: ");
                    io::stdout().flush()?;
                    let mut new_file_name = String::new();
                    io::stdin().read_line(&mut new_file_name)?;

                    self.create_file(&self.current_path.join(new_file_name.trim()))?;
                }
                5 => self.navigate_up(),
                6 => {
                    println!("Listing Directory with Properties:");
                    self.list_directory_with_properties()?;
                }
                7 => {
                    println!("Exiting...");
                    break;
                }
                _ => {
                    println!("Invalid choice. Try again.");
                }
            }
        }

        Ok(())
    }
}
