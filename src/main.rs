//TODOs:
//line 121
//line 132

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

use std::env; //Для чтения аргументов из командной строки
// и почему так "use std::env::args;" не работает??

const TEST_FILE_NAME: &str = "test file.txt";

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
		let mut f = File::open(TEST_FILE_NAME).unwrap();

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

fn scan_files_rec( scaning_directory: &path::Path ){

	let is_recursive_search = true;

	let files_in_scaning_directory: fs::ReadDir = fs::read_dir(scaning_directory).unwrap();

	for file in files_in_scaning_directory {
		let file = file.unwrap().path();
		//let metadata = fs::metadata(&file).unwrap();

		if fs::metadata(&file).unwrap().file_type().is_dir() {
			// println!("\tThis is dir!");
			if is_recursive_search {
				scan_files_rec(&file);
			}
		}else{
			println!("File: {}", file.display());
			// тут нужно записывать md5 файла в масив
		}
	}
}

fn main() {

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
	
	scan_files_rec(&directory_search);

	/*
	let mut strr2: String = String::new();

	let len = my_read_file(TEST_FILE_NAME, &mut strr2);
	println!("my_read_file: {} {}", len, strr2);
	*/

	//}
}