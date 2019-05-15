pub fn variables() {
    let name = "Rac";
	let mut age = "age";
	println!("name is {} age {}", name, age);
	age = "4";
	println!("name is {} age {}", name, age);
	const ID: i32 = 021;
	println!("ID {}", ID);
	
	let (has, have) = ("het", 3);
    println!("{}  {}", has, have); 	
}