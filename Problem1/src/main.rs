fn main() {
    println!("ProjectEuler.net Problem 1:\nSum of all multiples of 3 or 5 below 1000.");
    let target : u32 = 999;
    let sum : u32 = sum_divisible_by(3, target) + sum_divisible_by(5, target) - sum_divisible_by(15, target);
    println!("Answer: {}\n", sum);
}

fn sum_divisible_by(n : u32, target : u32) -> u32 {
	let p : u32 = target/n;
	return n*(p*(p+1))/2;
}