pub fn ye() {
    println!("from other file");
	println!("{} like to {} {}", "Ashley", "eat", "cake");
	println! (
	    "{0} has {1} {2}",
		"Eduard", "many", "commitments"
    ); 
	println! (
	    "If {yeet} loses we {agh}",
		yeet = "Arvin",
		agh = "riot!"
	);	
	println!("Binary: {:b} Hex: {:x} Octal: {:o}" ,9, 4, 33);
	
	//debug traits
	println!("{:?}", (44, false, "hi"));
	println!("{}", 10 * 10);
}	