/** Using the guide found here:
  * http://articles.leetcode.com/2012/01/palindrome-number.html
  * Define a function that can judge whether a number is a palindrome.
  * Cycle through all products of two 3-digit numbers
  * For each product, test if it is a palindrome.
  * If it is, and if it is bigger than previously discovered palindromes,
  * Store it in the dedicated variable.
 */

fn main() {
    println!("ProjectEuler.net Problem 4:\nFind the largest palindrome made from the product of two 3-digit numbers.\n");
    let mut palindrome : u32 = 0;
    let mut n1 : u32 = 0;
    let mut n2 : u32 = 0;
    for m1 in 100..999 {
        for m2 in 100..999 {
            if is_palindrome(m1*m2) && m1*m2 > palindrome {
                palindrome = m1*m2;
                n1 = m1;
                n2 = m2;
            }
        }
    }
    println!("Answer: {} * {} = {}", n1, n2, palindrome);
}

fn is_palindrome(mut x : u32) -> bool {
    let mut div : u32 = 1;
    while x / div >= 10 {div *= 10;}
    // Variable div will 1 followed by zeroes. Will be as many digits long as x.

    while x != 0 {
        let left : u32 = x / div;
        //Let the "left" digit be x / 10^n where n is x's length in digits
        let right : u32 = x % 10;
        //Let the "right" digit be the remainder after dividing x by 10.
        if left != right {return false;} //If they are a palindrome, l would always equal r.
        x = (x%div) / 10; //If we haven't ruled this out as not a palindrome yet, trim the first and last digits
        div /= 100; //...and shorten div by two digits.
    }
    return true;
}
