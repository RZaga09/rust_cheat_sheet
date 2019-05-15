pub fn loo () {
    let count = 9;
	
	loop {
	    count += 1;  //add one
		println!("{}", count);
		
		if count == 25{
		    break;   //break at 25
		}
	}
}