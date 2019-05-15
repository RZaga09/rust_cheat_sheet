//Structure is the collection of variables of different types under a single name for better handling

pub fn struc () {
    let mut c = Color {
	    red: 255,
		green: 0,
		blue: 0
	};	
	
	let mut g = col(255, 0, 0);
	
	let mut p = Person::new("jo" , "doe");
	
	    c.red = 200; //changes property
		g.0 = 12;
		println!("{} {} {}", c.red, c.green, c.blue);
		println!("{} {} {}", g.0, g.1, g.2);
		println!("{} {}", p.first, p.last);
		println!("{}", p.full_name());
		
		p.set_last_name("WIll");
		println!("{}", p.full_name());
		println!("{:?}", p.tup());
}

    struct Color {  //traditional struct
	    red: u8,
		green: u8,
		blue: u8
	}

	//tuple struct
	struct col (u8, u8, u8);
	
	struct Person {
	    first: String,
		last: String
	}

    impl Person {   //impl function
        fn new(fir: &str, las: &str) -> Person	{
		Person {
		    first: fir.to_string(),
			last: las.to_string(),
	}
	}
	    fn full_name(&self) -> String {   //&self - reference
		    format!("{} {}", self.first, self.last) //no semicolon. format is being returned. format like println! except notprinted
	}
        fn set_last_name(&mut self, last: &str) {
            self.last = last.to_string();  //allows last name  to br edited		
    }
        //name to tuple
		fn tup(self) -> (String, String) {
		    (self.first, self.last)
	}
}