//resizable arrays


pub fn vec () {
    let mut num: Vec<i32> = vec![1, 2, 3];
		println!("{:?}", num); //prints the entire array 
		
		num.push(44);
		num.push(33);
		println!("{:?}", num);
		num.pop(); //takes off last value
		println!("{:?}", num);

	
		for ddd in num.iter() {  //loops through values. iter = iteration
		    println!("{}", ddd);
		}
		
		for x in num.iter_mut() { //_mut allows us to modify
		    *x *= 2;  //multiply by 2
			}
			println!("{:?}", num);
		
}