fn main() {	
	let mut tank = Tank::new(100, 3);
	
	println!("The tank has {} liters. It will empty over the course of {} hours.", tank.liters, tank.hours_left());
	tank.leak_one_hour();
	tank.leak_one_hour();
	tank.leak_one_hour();
	println!("The tank has {} liters", tank.liters);
	println!("{:?}", tank);
}

#[derive(Debug)]
struct Tank {
	liters: i32,
	leak_per_hour: i32,
}

impl Tank {
	fn new(capacity: i32, leak_per_hour: i32) -> Tank {
		Tank {
			liters: capacity,
			leak_per_hour,
		}
	}
	
	fn hours_left(&self) -> i32 {
		self.liters / self.leak_per_hour
	}

	fn leak_one_hour(&mut self) {
		self.liters -= self.leak_per_hour;
	}
}
