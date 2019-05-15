//STRING - immutable, fixed legght, in memory
//growable, heap allocated

pub fn stringsss () {
    let mut hello = String::from("Hello");
	
	hello.push('W'); //string.push = adds to string (for char only)
	hello.push_str(" world"); //string.push_str = push for strings
	
	println! ("String: {} Length: {} Capacity {}", hello, hello.len(), hello.capacity()); 
	//string.len = length of string
	//string.capacity = capacity
	//string.is_empty = check if string is empty (true or false)
	
	println!("CONTAINS {}", hello.contains("world")); //.cotains("); = check if text is in string
	
	println!("Replace {}", hello.replace("world", "arv")); //string.replace ("", ""); = string but replaces chars
	
	//loop through string by whitespace (Separate segments of string into separate lines based on spaces indicated)
	for word in hello.split_whitespace() {
	println!("{}", word);
	}
	
	//create string with certain capacity (capacity:  how much memory the string allocated to hold its contents)
	let mut ss = String::with_capacity(5);
	ss.push_str("Hiiiiiiiii33");
	println!("{} {}", ss, ss.capacity());
	
	//assertion testing
	assert_eq!(12, ss.len()); //compares if 2 is equal to len of ss (ONLY shows error if it doesnt pass test)
} 