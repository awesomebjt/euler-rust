fn main() {
    println!("ProjectEuler.net Problem 2:\nSum of all even Fibbonaci numbers under 4 million.");
    let mut fib;
    let mut total = 0;
    let mut x = 1;
    let mut y = 1;
    loop {
    	if (x + y) > 4000000 {
    		break;
    	}
    	fib = x + y;
    	x = y;
    	y = fib;
    	if fib%2==0 {
    		total += fib;
    	}
    }
    println!("Answer: {}", total);
}
