pub fn cond () {
    let age = 18;
	let check_id: bool = false; //didnt check_id
	let knows_of_age = true; //knows perso is of age
	
	if age > 16 {
	    println!("wow");
	}	
	
	if age == 19 || check_id == false && knows_of_age {
	    println!("hellyeah");
	} else if age >= 21 && check_id {
	    println!("drink"); //greater than or equal to
	} else if age < 21 && check_id || knows_of_age { // || = or
        println!("no");	
    } else {
	    println!("d");
    }
	
	let off = if age <= 22 {"fals"} else {"rue"};
	println! ("{}", off);
}