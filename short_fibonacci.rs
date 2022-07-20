// creates a sequence containing the first six numbers in Fibonacci series
fn short_fibonacci() -> Vec<u8> {
	let mut sequence: Vec<u8> = Vec::new();
	for num in 0..6 {
		if num < 2 {
			sequence.push(num)
		} else {
			let val = sequence[(num-1) as usize] + sequence[(num-2) as usize];
			sequence.push(val);
		}
	}
	sequence
}

fn main() {
	println!("{:?}", short_fibonacci());
    for (pos, e) in short_fibonacci().iter().enumerate() {
        println!("Element at index {} is {}", pos, e);
    }
}
