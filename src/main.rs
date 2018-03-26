use std::fs;
use std::path;

fn main() {
	
	let this_dir = path::Path::new("./"); //относительно пути запуска программы
    let paths: fs::ReadDir = fs::read_dir(this_dir).unwrap();

    for path in paths {
    	let path = path.unwrap().path();
        let metadata = fs::metadata(&path).unwrap();

        println!("Name: {}; Type: {:?};"
        	, path.display()
        	, metadata.file_type()
        	);
    }

}