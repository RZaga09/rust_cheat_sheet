pub fn aah () {
    let x = 1; //default i32
	
	let y = 2.5; //default f64
	
	let z: i64 = 4234234;
	
	//find max size
	println! ("Max i32 is {}", std::i32::MAX);
	
	//boolean
	let active = false;
	
	println!("{:?}", (x, y, z, active));
    
	let yay: bool = 10 == 3;
	println!("{:?}", (x, y, z, active, yay));
	
	//char = '  (only one codept)
	let a1 = 'a';
	println!("{}", a1);
	let face = '\u{1F600}';//unicode
	println!("{}", face);
}