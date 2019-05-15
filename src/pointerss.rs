//point to resource in memory

pub fn po () {
    let arr1 = (1,2,3);
	let arr2 = arr1;
	
	println!("{:?}", (arr1, arr2));
	
	let vecc = vec![1,2,3];
	let veck = &vecc;    //& required for non primitive data types
	
	println!("{:?}", (veck, &vecc));
}