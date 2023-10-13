use std::io;
use rand::Rng; 

fn main() {
    // Task 1
    println!("Task 1: Hello, Rust!");

    // Task 2
    let name = String::from("Fareeha");
    println!("Hello, {}!", name);

    // Task 3
    println!("Task 3: Hello, to begin your temperature conversion, what's the unit of it:");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Error reading input");
    let t: i32 = temp.trim().parse().expect("Invalid input");

    println!("Choose conversion: 1. Celsius to Fahrenheit 2. Fahrenheit to Celsius");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let inp: i32 = input.trim().parse().expect("Invalid input");

    if inp == 1 {
        println!("Temperature in Celsius: {}", (t * 9 / 5) + 32);
    } else {
        println!("Temperature in Fahrenheit: {}", (t - 32) * 5 / 9);
    }

    // Task 4
    println!("Task 4: Factorial of 5 is: {}", fact(5));

    // Task 5
    let num = 2;
    if num % 2 == 0 {
        println!("Task 5: The number {} is even", num);
    } else {
        println!("Task 5: The number {} is odd", num);
    }

    // Task 6
    println!("Task 6: Enter a number to sum up to:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    let n: i32 = input.trim().parse().expect("Invalid input");
    let mut sum = 0;
    for i in 1..=n {
        if i != 1 {
            print!(" + ");
        }
        print!("{}", i);
        sum += i;
    }
    println!(" = {}", sum);

    // Task 7
    println!("Task 7:");
    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }

    // Task 8
    println!("Task 8:");
    palindrome("121".to_string());

    // Task 9
    println!("Task 9:");
    let num = rand::thread_rng().gen_range(1..20);
	println!("Random Number: {}", num);
	println!("");

	let mut guess = String::new();
	let mut flag = false;

	loop {
		println!("Enter a guess: ");
		std::io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		println!("You entered: {}", guess);
	
		let int_guess = guess.trim().parse::<i32>().unwrap();
	
		if int_guess == num {
			println!("You win!");
			flag = true;
		}
		else if int_guess > num {
			println!("Too big!");
		}
		else {
			println!("Too small!");
		}

		if flag {
			break;
		}

		guess.clear();
    }

    // Task 10
    println!("Task 10: Basic Calculator");
    basic_calculator();

    // Task 11
    println!("Task 11:");
    let arr: [i32; 5] = [50, 10, 12, 48, 61];
    let ans = largest_num(&arr);
    println!("Largest element is: {}", ans);

    // Task 12
    println!("Task 12:");
    let my_str = String::from("Hello, Beautiful!");
    let reversed_str = reverse_string(&my_str);
    println!("Reversed String: {}", reversed_str);

    // Task 13
    println!("Task 13:");
    generate_fibonacci_sequence(10);

    // Task 14
    println!("Task 14:");
    let num = 7;
    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
    // Task 15
    let my_string = "Hello".to_string();
    // Call the modify_string function, passing ownership of the string
    modify_string(&my_string.clone());
    println!("String outside the function: {}", my_string);
    println!("Borrowed inside main: {}", &my_string); println!();

    // Task 16
    println!("Task 16:");
    ownership_and_borrowing();

    // Task 17
    let input = "Hello, You!";
    let first_char = &input[0..1];
    let last_char = &input[input.len() - 1..];
    println!("First character: {}", first_char);
    println!("Last character: {}", last_char); println!();

    // Task 18
    println!("Task 18:");
    let string = "Hello World!";
    let substring = "l";
    let count = count_substring(string, substring); println!();
    // Task 19
    println!("Task 19:");
    reverse_sentence("Rust is fun and powerful");

    // Task 20 src/task_12
}

fn fact(n: u8) -> u8 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}

fn palindrome(s: String) {
    let reversed = s.chars().rev().collect::<String>();
    if s == reversed {
        println!("The string is a palindrome");
    } else {
        println!("The string is not a palindrome");
    }
}

fn basic_calculator() {
    // Input for num1
    println!("Enter num1: ");
    let num1: f64 = read_input();

    // Input for num2
    println!("Enter num2: ");
    let num2: f64 = read_input();

    println!("Enter operator (+, -, *, /): ");
    let operator = read_operator();

    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => {
            println!("Invalid operator.");
            f64::NAN
        }
    };

    println!("Result: {}", result);
}

fn read_input() -> f64 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        }
    }
}

fn read_operator() -> char {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let operator = input.trim();

        if operator.len() == 1 {
            return operator.chars().next().unwrap();
        }

        println!("Invalid input. Please enter a valid operator (+, -, *, /):");
    }
}

fn largest_num(arr: &[i32; 5]) -> i32 {
    let mut max = arr[0];
    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }
    max
}

fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn generate_fibonacci_sequence(n: u32) {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        print!("{} ", a);
        let temp = a;
        a = b;
        b = temp + b;
    }
    println!();
}

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..(num / 2 + 1) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

fn ownership_and_borrowing() {
    let my_string = "Hello".to_string();
    let borrowed = modify_string(&my_string);
    println!("String outside the function: {}", my_string);
    println!("Borrowed inside main: {}", borrowed);
}

fn modify_string(s: &str) -> String {
    let modified = format!("{} World!", s);
    println!("Inside the function: {}", modified);
    modified
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn count_substring(string: &str, substring: &str) -> usize {
    let mut count = 0;
    let mut start = 0;
    while let Some(positioning) = string[start..].find(substring) {
        count += 1;
        start += positioning + substring.len();
    }
    count
}

fn reverse_sentence(sentence: &str) {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let reversed: Vec<String> = words.iter().map(|word| word.chars().rev().collect()).collect();
    let reversed_sentence = reversed.join(" ");
    println!("Original: {}", sentence);
    println!("Reversed: {}", reversed_sentence);
}

// Task 20: You need to implement reading and counting words in a text file. The code for this task is not provided in your initial code.
