use std::io;
/*
6. Create a Rust program that calculates the sum of all integers from 1 to N,
where N is provided by the user.
7. Write a Rust program that prints numbers from 1 to 100. For multiples of 3,
print "Fizz" instead of the number. For multiples of 5, print "Buzz" instead of
the number. For numbers which are multiples of both 3 and 5, print
"FizzBuzz."
9. Create a Rust program that generates a random number between 1 and 100
and allows the user to guess it. Provide feedback on whether the guess is too
high or too low.
10.Develop a Rust program that acts as a basic calculator. It should allow the
user to enter two numbers and an operation (+, -, *, /) and display the result.

12.Implement a Rust function that takes a string as input and returns the string
reversed.
13.Write a Rust function that generates the Fibonacci sequence up to a specified
number of terms, N.
14.Create a Rust program that generates and prints all prime numbers up to a
given limit, N.
15.Develop a Rust program that demonstrates ownership and borrowing
concepts. Define a function that takes ownership of a string, modifies it, and
then borrows it back for printing.
16.Define a Rust function that takes two string references as parameters and
returns the longest one. Include lifetime annotations in the function signature.
17.Create a Rust program that takes a string from the user and prints its first and
last character using string slicing.
18.Implement a Rust function that takes a string and a substring as input and
returns the number of times the substring appears in the string.
19.Write a Rust function that takes a sentence as input and reverses the order of
words while preserving the order of characters within each word.
20.Create a Rust program that reads a text file and counts the occurrences of
each word, ignoring punctuation and case. Use string slicing to split the text
into words.
*/
fn main(){
println!("Hello, to begin your temperture conversion whats the unit of it:");
    let mut temp  = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Error to read that line.");
        let t: i32 = temp.trim().parse().expect("Not a valid number");

    println!("Write 1.CELICUS to FAREHEIT 2. FAREHEIT TO CELSIUS ");
    let mut input  = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error to read that line.");
        let inp: i32 = input.trim().parse().expect("Not a valid number");
    if inp == 1 {
        println!("Temp in celicus: {}", ((9/5)* t) + 32 );
    }
    else{
        println!("Temp in Fahrenheit: {}", ((5/9)* t) - 32 ); }
////////////////////////////////////////  
//11:
let arr:[i32;5] = [50,10,12,48,61]; 
let ans =largest_num(arr);
println!("Largest element is: {}", ans);

// 17.Create a Rust program that takes a string from the user and prints its first and last character using string slicing.

    // Define a string literal.
    let input = "Hello, World!";

    // Get the first character using string slicing.
    let first_char = &input[0..1];

    // Get the last character using string slicing.
    let last_char = &input[input.len() - 1..];

    // Print the first and last characters.
    println!("First character: {}", first_char);
    println!("Last character: {}", last_char);
}



 //4. Implement a Rust function that calculates the factorial of a given non-negative integer using recursion.
fn fact(n: u8) -> u8{
    if n == 0{
        return 1
    }
    else{
        n * fact(n-1)
    }
}
//8. Implement a Rust function that checks if a given string is a palindrome (reads the same forwards and backwards).
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut y = x;
    let mut z: i32 = 0;
    while y>0 {
        z = z*10 + y%10;
        y/=10;
    }
    z==x
}
//11. Write a Rust function that finds and returns the largest number in an array of integers  
fn largest_num(arr:[i32;5]) -> i32{

    let mut large:i32 = 0;
    let mut i:usize = 0;
    
    large=arr[0];
    while i<arr.len()
    {
         if large < arr[i] {
             large = arr[i]
         }
         i = i + 1;
    }
    large

}

