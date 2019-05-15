use std::env;

pub fn comm () {
    let args: Vec<String> = env::args().collect();
	
	println!("{:?}", args); 
	//prints out ["target\\debug\\hello_world.exe"]. if "cargo run hello" prints out ["target\\debug\\hello_world.exe", "hello"] if "cargo run hello world" prints out ["target\\debug\\hello_world.exe", "hello", "world"]
	let command = args[1].clone();
	println!("{}", command);  //prints "hello"
	
	let name = "Brad";
	let status = "100";
	if command == "hello" {
	    println!("{}", name); //pritns "Brad" to "cargo run hello"
	} else if command == "status" {
	    println!("{}", status);
	}
}