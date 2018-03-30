//use std::io::prelude::*; // ?

use std::fs; // read_dir()
use std::path;

//https://crates.io/crates/md5
extern crate md5;

// https://doc.rust-lang.org/beta/std/io/struct.BufReader.html
// https://doc.rust-lang.org/std/fs/struct.File.html
use std::fs::File;
use std::io::Read; //File::read_to_end()
use std::str; //str::from_utf8()

//Похоже я не обрабатываю ошибки
fn my_read_file( file_name: &str, mut buf_str: &str) -> usize {

	let mut f = File::open(file_name).unwrap();
	//Почему он ищет этот файл в корневой папке? Мб карго перенаправляет...
	//вместо ?, unwrap() - как исправить? и вообще wtf?
	//Почему f должен быть mut?

	let mut buffer = Vec::<u8>::new(); // А тут нужен mut?

	let len = f.read_to_end(&mut buffer).unwrap(); // принимает &mut Vec<u8>

	buf_str = str::from_utf8(&buffer).unwrap(); // buf_str - по 10 раз создаю копии только для type cast

	// println!("!!!: {:?} {}", len, buf_str);
	// assert_eq!(len, 5);
	// assert_eq!(buf_str, "hello");

	return len;
}

#[cfg(test)]
mod ma_testing {
	use super::*;

	#[test]
	fn it_works() {
		assert_eq!(4, 2+2);
	}

	#[test]
	//Похоже я не обрабатываю ошибки
	fn read_file_test(){
		let mut f = File::open("log.txt").unwrap();
		//Почему он ищет этот файл в корневой папке? Мб карго перенаправляет...
		//вместо ?, unwrap() - как исправить? и вообще wtf?
		//Почему f должен быть mut?

		let mut buffer = Vec::<u8>::new(); // А тут нужен mut?

		let len = f.read_to_end(&mut buffer).unwrap(); // принимает &mut Vec<u8>

		let buf_str = str::from_utf8(&buffer).unwrap(); // buf_str - по 10 раз создаю копии только для type cast

		// println!("!!!: {:?} {}", len, buf_str);
		assert_eq!(len, 5);
		assert_eq!(buf_str, "hello");
	}
	
	#[test]
	fn md5_test(){
		let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
		assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
	}

	#[test]
	fn get_md5_of_file_test(){
		// let digest = ;
		assert_eq!(format!("{:x}", md5::compute(b"hello")), "5d41402abc4b2a76b9719d911017c592");
	}

}


fn search_files_rec( this_dir: &path::Path ){

	let is_recursive_search = true;

	let paths: fs::ReadDir = fs::read_dir(this_dir).unwrap();

	for path in paths {
		let path = path.unwrap().path();
		let metadata = fs::metadata(&path).unwrap();

		if metadata.file_type().is_dir() {
			// println!("\tThis is dir!");
			if is_recursive_search {
				search_files_rec(&path);
			}
		}else{
			println!("File: {}", path.display());
		}
	}
}

fn main() {

	let this_dir = path::Path::new("./"); //".\\" - нет такой директории пишет Linux //относительно пути запуска программы
	// search_files_rec(&this_dir);

	let strr : &str;
	let len = my_read_file("log.txt", strr);

}