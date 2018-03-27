use std::fs;
use std::path;

fn search_files_rec( this_dir: &path::Path ){

	let isRecursiveSearch = true;

	let paths: fs::ReadDir = fs::read_dir(this_dir).unwrap();

	for path in paths {
		let path = path.unwrap().path();
		let metadata = fs::metadata(&path).unwrap();

		if metadata.file_type().is_dir() {
			// println!("\tThis is dir!");
			if isRecursiveSearch {
				search_files_rec(&path);
			}
		}else{
			println!("File: {}", path.display());
		}
	}
}

fn main() {

	let this_dir = path::Path::new(".\\"); //относительно пути запуска программы

	search_files_rec(&this_dir);
}