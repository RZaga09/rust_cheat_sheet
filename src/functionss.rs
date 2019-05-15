pub fn func () {
    greeting("hello", "AJ");
	
	//bind nf values to vars
	let sum = add(4, 9);
	println!("{}", sum);
	
	//closure
	let numm: i32 = 10;
	let adyeah = |num: i32, nu:i32| num + nu + numm; //returns num+nu+numm
	println!("{}", adyeah(3, 3));
}

fn greeting(greet: &str, name: &str) {   //greet = hello, name = AJ
    println!("{}  {}", greet, name);
}

fn add(num: i32, nu: i32) -> i32 {
    num + nu  //no semicolon to show that we return this
}