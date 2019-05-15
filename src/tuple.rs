//grp together values of diff types
//max 12 elements

pub fn tup () {
    let pers: (&str, &str, i8) = ("Rac", "Gerard", 33); //&str = literal string
	println!("{} {} {}", pers.0, pers.1, pers.2);
}