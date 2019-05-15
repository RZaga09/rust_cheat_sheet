//types with few definite vales

enum Movement {
    //variants
	Up,
	Down,
	Left,
	Right
}

fn movel(m: Movement) {
    //perform action depending on info
	match m {
	    Movement::Up => println!("UP"),
		Movement::Down => println!("Down"),
		Movement::Left => println!("Left"),
		Movement::Right => println!("Right")
	}
} 

pub fn enn() {
    let ava1 = Movement::Left;
	let ava2 = Movement::Right;
	let ava3 = Movement::Down;
	let ava4 = Movement::Up;
	
	movel(ava1);
	movel(ava2);
	movel(ava3);
	movel(ava4);
}