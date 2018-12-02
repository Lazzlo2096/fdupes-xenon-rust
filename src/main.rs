use std::fs; //read_dir File
use std::path; // Path
use std::str; // str::from_utf8()

// https://doc.rust-lang.org/beta/std/io/struct.BufReader.html
use std::io::Read; //File::read_to_end()

// https://crates.io/crates/md5
extern crate md5;

use std::collections::HashMap;

extern crate clap; // to parse command line args
use clap::{Arg, App};

fn compute_files_hash_and_place_in_dict(
	scaning_directory: &path::Path,
	hash_paths_dict: &mut HashMap<[u8; 16], Vec<path::PathBuf>>,
	is_recursive_scan: bool
){
	let files_in_scaning_directory = fs::read_dir(scaning_directory).unwrap(); // <fs::ReadDir>

	for entry in files_in_scaning_directory {

		let entry = entry.unwrap().path(); //std::path::PathBuf

		// entry.metadata()
		if fs::metadata(&entry).unwrap().file_type().is_dir() {
			// println!("\tThis is dir!");
			if is_recursive_scan {
				compute_files_hash_and_place_in_dict(&entry, hash_paths_dict, is_recursive_scan);
			}
		}else{

			//открыть файл==========
			let mut f = fs::File::open(&entry).unwrap();
			//не обрабатываю ошибки открытия (файл занят и т.д.)

			//Почему f должен быть mut? нужно открыть только для чтения
			// ! А НУЖНО ЛИ ЗАКРЫВАТЬ ФАЙЛ???

			let mut buffer = Vec::<u8>::new();
			f.read_to_end(&mut buffer);
			//======================
			
			//посчитать хеш=========
			let md5::Digest(hash) = md5::compute(buffer); // [u8; 16]
			/*println!("File: {}", entry.display());  // TRACE
			let hash_str = format!("{:x}", md5::Digest(hash));
			println!("File: {:?} \t {}", entry.file_name().expect("the world is ending"), hash_str);*/
			//======================

			//хеш мап или B-tree? // https://doc.rust-lang.org/1.0.0/std/collections/index.html
			//=======записывать хеш md5 файла в в дикшонари(хеш, вектор с путями)=====
			if hash_paths_dict.contains_key(&hash) {
				//append (push) :
					// такой безусловный unwrap, потому что точто есть такой кей
					hash_paths_dict.get_mut(&hash).unwrap().push( entry );
				
				// уже тут можно сдлеать какую нибудь оптимизацию по выделению повторок //а нужна ли она??
			} else {
				hash_paths_dict.insert(hash, vec![entry]);
				//println!("{:?}", hash_paths_dict);
			}
			//========================================================================
		}
	}
}

fn main() {
	//==BEGIN===Args Processer==================
	let matches = App::new("fdupes-xenon")
		.version("0.1.1")
		//.author("lazzlo2096 <lazzlo2096@yandex.ru>")
		.about("Duplicates finder on Windows and Linux.\nRust version of fdupes written from scratch.")
		.arg(Arg::with_name("PATHS") 
			.help("Sets the paths there will be find duplicates")
			.required(true)
			.index(1)
			//.takes_value(true) //why work without it?
			.multiple(true) // ?? // https://docs.rs/clap/2.32.0/clap/struct.Arg.html#method.multiple
			) 
		.arg(Arg::with_name("RECURS")
			.help("Обходить ли папки рекурсивно")
			.short("r") )
		//.arg(Arg::with_name("v") // verbose
		//	.short("v")
		//	.multiple(true)
		//	.help("Sets the level of verbosity") )
		.get_matches();

	//==========testing=========
	/*
	assert!(matches.is_present("PATHS"));
	assert_eq!(matches.occurrences_of("PATHS"), 2);
	let files: Vec<_> = matches.values_of("PATHS").unwrap().collect();
	assert_eq!(files, ["./for tests", "./src"]);
	*/

	// assert!(matches.is_present("verbose"));
	// assert_eq!(matches.occurrences_of("verbose"), 3);
	//==END=====testing=========

	//====paths=================
	let paths_from_args: Vec<_> = matches.values_of("PATHS").unwrap().collect();
	let mut paths_for_searching: Vec<_> = Vec::new(); // ПОЧЕМУ РАБОТАЕТ БЕЗ ТИПА?!?!? ЛОЛ
	//let this_dir = path::Path::new("./");
	//^ ".\\" - нет такой директории пишет Linux
	//^ это относительно пути запуска программы
	//v [FIXIT] А ЕСЛИ value не корректен как путь??
	//v А что если нет одного из папок в списке? ТО всё должно зафелиться!
	for str_path in paths_from_args { // map it?
		paths_for_searching.push(  path::Path::new( str_path )  ); // path::Path::new( &(qwer) ); //qwer: &str
	}
	//==END===paths=================

	let is_recursive_scan = matches.is_present("RECURS");

	//==END===Args Processer===================

	// почему у Path не известен размер при компиляции? глупый вопрос!
	let mut hash_paths_dict: HashMap<[u8; 16], Vec<path::PathBuf>> = HashMap::new();

	for path_for_searching in paths_for_searching {
		compute_files_hash_and_place_in_dict( path_for_searching, &mut hash_paths_dict, is_recursive_scan);
	}
	
	for (_key, val) in hash_paths_dict.iter() {
		if val.len()>1 {
			println!("[ hash: {:?}] --------", _key); //  {:x}
				 //size // hash // max common folder ?
			//если префикс ./ то не покаpывать его...
			for path_to_file in val {
				println!("{:?}", path_to_file);
			}
			// WHY "\\" ? : ["./for tests/file (3).txt", "./for tests/folder 2\\file (3).txt"] in windows
		}
	}
}


#[cfg(test)]
mod ma_testing {
	use super::*;

	const TEST_FILE_NAME: &str = "test file.txt";
	const DIR_FOR_TESTS: &str = "for tests";

	#[test]
	fn read_file_test(){
		let mut f = fs::File::open( path::Path::new(DIR_FOR_TESTS).join(TEST_FILE_NAME) ).unwrap();

		let mut buffer = Vec::<u8>::new(); 

		let len = f.read_to_end(&mut buffer).unwrap(); 

		let buf_str = str::from_utf8(&buffer).unwrap();

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
		//чё я тут намутил...
		let mut buffer = Vec::<u8>::new(); 
		let _len = fs::File::open( path::Path::new(DIR_FOR_TESTS).join(TEST_FILE_NAME)).unwrap().read_to_end(&mut buffer).unwrap(); 

		let qwerty = str::from_utf8(&buffer).unwrap();
		let file_content = String::from(qwerty);

		assert_eq!(format!("{:x}", md5::compute(file_content.into_bytes())), "5d41402abc4b2a76b9719d911017c592");
	}
}