//fiizbuzz challenge

pub fn loo () {
    let mut count = 0;
	//loop {
	//    count += 1;  //add one
	//	println!("{}", count);
	//	
	//	if count == 100{
	//	    break;   //break at 100
	//	}
	
	//If divisible by 3 print "fizz" if divisible by 5 print "buzz" if by both print "fizbuzz"
	while count <= 100 {
	    if count % 15 == 0 {  //% - modulus (finds remainder after dividing)
		println!("fizzbuzz");
		} else if count % 3 == 0 {
        println!("fizz");
		} else if count % 5 == 0 {
		println!("buzz");
		} else {
		println!("{}", count);
		}
		count += 1;  //add one;
	}
	
	for x in 0..100 {
	 if x % 15 == 0 {  //% - modulus (finds remainder after dividing)
		println!("fizzbuzz");
		} else if x % 3 == 0 {
        println!("fizz");
		} else if x % 5 == 0 {
		println!("buzz");
		} else {
		println!("{}", x);
		}
	}
}
