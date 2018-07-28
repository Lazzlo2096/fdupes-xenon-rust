//use std::io::prelude::*; // ?

// https://doc.rust-lang.org/std/fs/struct.File.html
// use std::fs::{*, self};
use std::fs; //read_dir File
use std::path; // Path
use std::str; //str::from_utf8()

// https://doc.rust-lang.org/beta/std/io/struct.BufReader.html
use std::io::Read; //File::read_to_end()

// https://crates.io/crates/md5
extern crate md5;

use std::collections::HashMap;

extern crate clap; // to parse command line args
use clap::{Arg, App};

const TEST_FILE_NAME: &str = "test file.txt";
const DIR_FOR_TESTS: &str = "for tests";
//const TEST_FILE_NAME_path: &path::Path = &Path::new("test file.txt"); //почему не компилиться?
//const DIR_FOR_TESTS_path: &path::Path = &Path::new("for tests"); // "."

//Похоже я не обрабатываю ошибки
fn my_read_file( file_name: &str, buf_str: &mut String) -> usize {
//Вообщем я реализвал наконец-то эту функцию, но наверно зазря.. потому что оказалось что и без меня есть read_to_string()

	let mut f = fs::File::open(file_name).unwrap();
	//Почему он ищет этот файл в корневой папке? Мб карго перенаправляет...
	//вместо ?, unwrap() - как исправить? и вообще wtf?
	//Почему f должен быть mut?

	let mut buffer: String = String::new();
	//^ А тут нужен mut? нужен, а зачем?
	let len = f.read_to_string(&mut buffer).unwrap();
	//^ принимает &mut String

	*buf_str = buffer;

	// println!("read_to_string: {:?} {}", len, buf_str);
	// assert_eq!(len, 5);
	// assert_eq!(buf_str, "hello");

	return len;
}

fn scan_files_hashes_rec( scaning_directory: &path::Path ){

	let is_recursive_scan = true;

	// or enrtries
	let files_in_scaning_directory = fs::read_dir(scaning_directory).unwrap(); //<fs::ReadDir>

	for entry in files_in_scaning_directory {

		let entry = entry.unwrap().path();
		//let metadata = fs::metadata(&entry).unwrap();

		// entry.metadata()
		if fs::metadata(&entry).unwrap().file_type().is_dir() {
			// println!("\tThis is dir!");
			if is_recursive_scan {
				scan_files_hashes_rec(&entry);
			}
		}else{
			//v тут нужно записывать md5 файла в масив
			//открыть===============
			//v не обрабатываю ошибки??
			let mut f = fs::File::open(&entry).unwrap();
			//Почему f должен быть mut?
			// ! А НУЖНО ЛИ ЗАКРЫВАТЬ ФАЙЛ???

			let mut buffer = Vec::<u8>::new();
			f.read_to_end(&mut buffer);
			//======================
			
			//посчитать хеш=========
			// let hash: md5::Digest = md5::compute(buffer); // Digest( [u8; 16] )
			let md5::Digest(hash) = md5::compute(buffer); // [u8; 16]
			let hash_str = format!("{:x}", md5::Digest(hash));
			//добавить хеш в дикшонари (путь, хеш) - мб ключ по хешу (хеш мап или B-tree?)
			 // или типа (hash , (указатель на) вектор с путями)
			//======================

			// let mut strr2: String = String::new();
			// let len = my_read_file(TEST_FILE_NAME, &mut strr2);
			// println!("my_read_file: {} {}", len, strr2);

			// println!("File: {}", entry.display());
			println!("File: {:?} \t {}", entry.file_name().expect("the world is ending"), hash_str);
		}
	}
}

fn main() {

	let matches = App::new("fdupes-xenon")
	.version("0.1.0")
	//.author("lazzlo2096 <lazzlo2096@yandex.ru>")
	.about("Duplicates finder on Windows and Linux.\nRust version of fdupes. Written from scratch.")
	.arg(Arg::with_name("PATH")
		.help("Sets the path there will be findes duplicates")
		.required(true)
		.index(1) )
	// .arg(Arg::with_name("v")
	// 	.short("v")
	// 	.multiple(true)
	// 	.help("Sets the level of verbosity") )
	.get_matches();

	// почему HashMap не приемлет md5::Digest ? и к тому же думаю у строки сравнение на равенство дольше
	// почему у Path не известен размер при компиляции?
	//дикшонари его мы будем передавать в scan_files_hashes_rec
	let hash_paths_dics: HashMap<&str, Vec<&path::Path>> = HashMap::new();

	//let this_dir = path::Path::new("./");
	//^ ".\\" - нет такой директории пишет Linux
	//^ это относительно пути запуска программы / WAT?

	//v А ЕСЛИ не корректен как путь??
	let qwer: &str = matches.value_of("PATH").unwrap();
	//println!("{:?}",  qwer);

	let directory_search = path::Path::new( &(qwer) );
	//^ что если я выйду за передлы массива? Что будет если я передам пустую строку?
	
	// let test_dir = path::Path::new( DIR_FOR_TESTS );
	// scan_files_hashes_rec(&test_dir);
	scan_files_hashes_rec(&directory_search);

}

#[cfg(test)]
mod ma_testing {
	use super::*;

	#[test]
	//Похоже я не обрабатываю ошибки
	fn read_file_test(){
		let mut f = File::open( Path::new(DIR_FOR_TESTS).join(TEST_FILE_NAME) ).unwrap();

		let mut buffer = Vec::<u8>::new(); 

		let len = f.read_to_end(&mut buffer).unwrap(); 

		let buf_str = str::from_utf8(&buffer).unwrap();

		assert_eq!(len, 5);
		assert_eq!(buf_str, "hello");
	}

	#[test]
	fn my_read_file_test(){

		let mut strr2: String = String::new();

		let len = my_read_file(TEST_FILE_NAME, &mut strr2);

		// println!("my_read_file: {} {}", len, strr2);
		assert_eq!(len, 5);
		assert_eq!(strr2, "hello");
	}
	
	#[test]
	fn md5_test(){
		let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
		assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
	}

	#[test]
	fn get_md5_of_file_test(){
		//чё я тут намутил...
		let mut buffer = Vec::<u8>::new(); 
		let len = File::open(TEST_FILE_NAME).unwrap().read_to_end(&mut buffer).unwrap(); 

		let qwerty = str::from_utf8(&buffer).unwrap();
		let file_content = String::from(qwerty);

		assert_eq!(format!("{:x}", md5::compute(file_content.into_bytes())), "5d41402abc4b2a76b9719d911017c592");
	}

	#[test] //зависит от теста my_read_file_test()
	fn get_md5_of_file_opened_by_my_func_test(){

		let mut file_content: String = String::new();
		my_read_file(TEST_FILE_NAME, &mut file_content);

		assert_eq!(format!("{:x}", md5::compute(file_content.into_bytes())), "5d41402abc4b2a76b9719d911017c592");
	}
}