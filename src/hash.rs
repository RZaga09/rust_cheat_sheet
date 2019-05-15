use std::collections::HashMap;

pub fn hassh () {
    let mut marks = HashMap::new();
	
	//adds values
	marks.insert("English", 98);   //subject => value
	marks.insert("Science", 96);
	marks.insert("Mathematics", 75);
	
	//find length
	println!("{} subjects", marks.len());
	
	//finding grade
	match marks.get("English") {
	    Some(mark) => println!("Grade is {}", mark), //Some(mark) if the mark mentioned is found
		None => println!("NO") //none- if the subject does not exist
	}
	
	//remove value
	marks.remove("English");
	
	//loops through hashmap
	for (subject, mark) in &marks {  //for (key, value)
	    println! ("{} {}", subject, mark);   //doesnt print english grade cause we removed it
	}
	
	//check for value
	println!("do u have {}", marks.contains_key("English")); //prints true or false
}