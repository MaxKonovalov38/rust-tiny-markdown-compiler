use std::path::Path;
use std::fs::File;
use std::io::{BuRead, BuReader};

fn get_title() -> String {
	// Возвращает строку - 'короткий баннер'
	let mut the_title = String::from(env!("CARGO_PKG_NAME"));
	the_title.push_str(" (v");
	the_title.push_str(env!("CARGO_PKG_VERSION"));
	the_title.push_str("), ");
	the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
	return the_title;
}

fn parse_markdown_file(_filename: &str) {
	// Будет вызываться, когда нам передадут md-файл через CL
	print_short_banner();
	println!("[ INFO ] Trying to parse {}...", _filename);

	// Создать переменную path из имени _filename
	let input_filename = Path::new(_filename);

	// Попытаюсь открыть файл
	let file = File::open(&input_filename)
		.expect("[ ERROR ] Failed to open file!");

	let mut _ptag: bool = false;
	let mut _htag: bool = false;

	let mut tokens: Vec<String> = Vec::new();

	let reader = BufReader::new(file);
}

fn print_short_banner() {
	// Выведет название, версию и описание
	println!("{}", get_title());
}

fn print_long_banner() {
	// Выведет короткий баннер плюс пример
	print_short_banner();
	let mut line = String::from("Written by: ");
	line.push_str(env!("CARGO_PKG_AUTHORS"));
	line.push_str("\nHomepage: ");
	line.push_str(env!("CARGO_PKG_HOMEPAGE"));
	line.push_str("\nUsage: tinymd <somefile>.md");
	println!("{}", line);
}

fn usage() {
	print_long_banner();
}

fn main() {
	// Работа с аргументами CL
	let args: Vec<String> = std::env::args().collect();
	match args.len() {
		2 => parse_markdown_file(&args[1]),
		_ => {
			println!("[ ERROR ] Invalid invocation (you done goofes!)");
			usage();
		}
	}
}