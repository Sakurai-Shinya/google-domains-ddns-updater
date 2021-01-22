use std::env;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::fs::File;
use std::thread;

use thread::JoinHandle;

fn main() {
	show_start_message();
	let domain_list = get_domain_list();
	let ip = get_ip_address();
	println!("IP: {}",ip);
	let mut thread_list = Vec::new();
	for domain in domain_list {
		let ip = ip.clone();
		let thread = thread::spawn(move || {
			println!("更新開始... ドメイン: {}",domain.domain);
			let client = reqwest::blocking::Client::new();
			let a = format!("https://{}:{}@domains.google.com/nic/update?hostname={}&myip={}",domain.user_name,domain.password,domain.domain,ip);
			let res = client.post(&a).header("Content-Type","application/x-www-form-urlencoded").body("").send().unwrap().text().unwrap();
			println!("{}の結果: {}",domain.domain,res);
		});
		thread_list.push(thread);
	}
	wait_for_thread_complete(thread_list);
}

fn show_start_message() {
	println!();
	println!("Google Domains DDNS Updater");
	println!("version 0.1.0");
	println!();
}

fn get_domain_list() -> Vec<Domain> {
	let mut domain_list = Vec::new();
	let exe_path = env::current_exe().unwrap();
	let parent_path = exe_path.parent().unwrap();
	let file = File::open(parent_path.join("domainlist")).unwrap();
	let mut reader = BufReader::new(file);
	loop {
		let mut line = String::new();
		let read_size = reader.read_line(&mut line).unwrap();
		if read_size==0 {
			break;
		}
		let line = line.trim();
		if line.len() != 0 && !line.starts_with("#") {
			let domain_array: Vec<&str> = line.split(",").collect();
			let domain = Domain{domain:domain_array[0].to_string(),user_name:domain_array[1].to_string(),password:domain_array[2].to_string()};
			domain_list.push(domain);
		}
	}
	domain_list
}

fn get_ip_address() -> String {
	reqwest::blocking::get("http://checkip.amazonaws.com/").unwrap().text().unwrap()
}

fn wait_for_thread_complete(threads:Vec<JoinHandle<()>>){
	for thread in threads {
		thread.join().unwrap();
	}
}

struct Domain {
	domain: String,
	user_name: String,
	password: String,
}