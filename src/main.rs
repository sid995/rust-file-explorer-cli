mod file_explorer;

use file_explorer::FileExplorer;

fn main() {
    let mut file_explorer = FileExplorer::new();
    file_explorer.run().expect("Error running file explorer");
}