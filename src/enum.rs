//types with few definite vales

enum Movement {
    //variants
	Up,
	Down,
	Left,
	Right
}

fn move(m: Movement) {
    //perform action depending on info
	match = {
	    Movement::Up => println!("UP"),
		Movement::Down => println!("Down"),
		Movement::Left => println!("Left"),
		Movement::Right => println!("Right"),
	}
}

pub fn enn () {
    let ava1 = Movement::Left;
	let ava2 = Movement::Right;
	let ava3 = Movement::Down;
	let ava4 = Movement::Up;
	
	move.avatar(ava1);
	move.avatar(ava2);
	move.avatar(ava3);
	move.avatar(ava4);
}