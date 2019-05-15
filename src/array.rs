//fixed list elements are same data type
use std::mem;

pub fn arra () {
    let mut num: [i32; 5] = [1, 2, 3, 4, 5];//[type; array length]
	println!("{:?}", num); //prints the entire array (wont work if not same no of vars as length indicated)
	
	num[0] = 55; //replace values
	
	println!("{0}, {}", num[0]); //both brackets print "55, 55"
	println!("{}", num.len()); //length
    println!("{}", mem::size_of_val(&num)); //memory
	
	//also: std::mem::size_of_val(&num)
	
	//get slice
	let slice: &[i32] = &num[1..4];
	println!("{:?}", slice); //array of vales 1-3
}