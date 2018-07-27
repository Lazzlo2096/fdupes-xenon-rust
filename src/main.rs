//TODOs:
//line 121
//line 132

//use std::io::prelude::*; // ?

use std::fs; // read_dir()
use std::path;
use std::path::Path;

//https://crates.io/crates/md5
extern crate md5;

// https://doc.rust-lang.org/beta/std/io/struct.BufReader.html
// https://doc.rust-lang.org/std/fs/struct.File.html
use std::fs::File;
use std::io::Read; //File::read_to_end()
use std::str; //str::from_utf8()


use std::collections::HashMap;

use std::env; //Для чтения аргументов из командной строки
// и почему так "use std::env::args;" не работает??


const TEST_FILE_NAME: &str = "test file.txt";
const TEST_FILE_PATH: &str = "for tests"; // "."
//const TEST_FILE_NAME_path: &path::Path = &Path::new("test file.txt"); //почему не компилиться?
//const TEST_FILE_PATH_path: &path::Path = &Path::new("for tests"); // "."

//Похоже я не обрабатываю ошибки
fn my_read_file( file_name: &str, buf_str: &mut String) -> usize {
//Вообщем я реализвал наконец-то эту функцию, но наверно зазря.. потому что оказалось что и без меня есть read_to_string()

	let mut f = File::open(file_name).unwrap();
	//Почему он ищет этот файл в корневой папке? Мб карго перенаправляет...
	//вместо ?, unwrap() - как исправить? и вообще wtf?
	//Почему f должен быть mut?

	let mut buffer: String = String::new(); // А тут нужен mut? нужен, а зачем?
	let len = f.read_to_string(&mut buffer).unwrap(); // принимает &mut String

	*buf_str = buffer;

	// println!("read_to_string: {:?} {}", len, buf_str);
	// assert_eq!(len, 5);
	// assert_eq!(buf_str, "hello");

	return len;
}

#[cfg(test)]
mod ma_testing {
	use super::*;

	#[test]
	//Похоже я не обрабатываю ошибки
	fn read_file_test(){
		let mut f = File::open( Path::new(TEST_FILE_PATH).join(TEST_FILE_NAME) ).unwrap();

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

fn scan_files_hashes_rec( scaning_directory: &path::Path ){

	let is_recursive_scan = true;

	let files_in_scaning_directory: fs::ReadDir = fs::read_dir(scaning_directory).unwrap();

	for file in files_in_scaning_directory {
		let file = file.unwrap().path();
		//let metadata = fs::metadata(&file).unwrap();

		if fs::metadata(&file).unwrap().file_type().is_dir() {
			// println!("\tThis is dir!");
			if is_recursive_scan {
				scan_files_hashes_rec(&file);
			}
		}else{
			//v тут нужно записывать md5 файла в масив

			//открыть===============
			let mut f = File::open(&file).unwrap(); //не обрабатываю ошибки??
			//Почему f должен быть mut?
			// ! А НУЖНО ЛИ ЗАКРЫВАТЬ ФАЙЛ???

			let mut buffer = Vec::<u8>::new();
			f.read_to_end(&mut buffer).unwrap();
			//======================
			
			//посчитать хеш=========
			let hash = md5::compute(buffer);
			let hash_str = format!("{:x}", hash);
            //добавить хеш в дикшонари (путь, хеш) - мб ключ по хешу (хеш мап или B-tree?)
             // или типа (hash , (указатель на) вектор с путями)
			//======================

			// let mut strr2: String = String::new();
			// let len = my_read_file(TEST_FILE_NAME, &mut strr2);
			// println!("my_read_file: {} {}", len, strr2);

			// println!("File: {}", file.display());
			println!("File: {:?} \t {}", file.file_name().expect("the world is ending"), hash_str);
		}
	}
}

fn main() {

    // почему HashMap не приемлет md5::Digest ? и к тому же думаю у строки сравнение на равенство дольше
    // почему у Path не известен размер при компиляции?
    let hash_paths_dics: HashMap<&str, Vec<&path::Path>> = HashMap::new(); //дикшонари его мы будем передавать в scan_files_hashes_rec

	//v получить директорию, иначе ошибка
	let args: Vec<String> = env::args().collect(); 
	//^ список из String, первый аргумент абс путь самой программы
    //v если только один аргумент, то вывести help
    //v А что если этот аргумент несуществующий путь или вообще не корректен как путь??
    if args.len() <= 1 {
    	let help_page = "use it this way:\n    xfdupes-xenon <path>";
    	println!("{}", help_page);
    }
    //else {
    	
	//let this_dir = path::Path::new("./");
	//^ ".\\" - нет такой директории пишет Linux
	//^ это относительно пути запуска программы / WAT?

	let directory_search = path::Path::new( &(args[1]) );
	//^ что если я выйду за передлы args? Что будет если я передам пустую строку? вне else, лул
	
	// let test_dir = path::Path::new( TEST_FILE_PATH );
	// scan_files_hashes_rec(&test_dir);
	scan_files_hashes_rec(&directory_search);

	/*
	let mut strr2: String = String::new();

	let len = my_read_file(TEST_FILE_NAME, &mut strr2);
	println!("my_read_file: {} {}", len, strr2);
	*/

	//}
}