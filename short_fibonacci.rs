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
    for (pos, e) in short_fibonacci().iter().enumerate() {
        println!("Element at index {} is {}", pos, e);
    }
}


#[test]
fn test_sequence_length() {
	assert_eq!(short_fibonacci().len(), 6)
}

#[test]
fn test_fifth_number_in_sequence() {
	assert_eq!(short_fibonacci()[4], 3)
}
