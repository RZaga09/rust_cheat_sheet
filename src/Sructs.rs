//create diff dtat types

pub fn struc () {
    let mut c = Color {
	    red: 255,
		green: 0,
		blue: 0
	};	
		println!("{} {} {}", c.red, c.green, c.blue);
}

    struct Color {  //traditional struct
	    red: u8,
		green: u8,
		blue: u8
	}
