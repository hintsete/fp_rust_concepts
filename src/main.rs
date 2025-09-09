//recusrsion in fp, preferred over loops to maintain immutability and purity
fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}

// Map in fp transforms each element in a collection using a pure function.
fn map<T, U, F>(vec: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    vec.into_iter().map(f).collect()
}

// pure functions Same input, same output, no side effects.
fn square(n: i32) -> i32 {
    n * n
}

// function composition f(g(x)).
fn compose<T, U, V, F, G>(vec: Vec<T>, f: F, g: G) -> Vec<V>
where
    F: Fn(U) -> V,
    G: Fn(T) -> U,
{
    vec.into_iter().map(|x| f(g(x))).collect()
}

// sum in fp Reduces a collection to a single value using a pure function.
fn sum(vec: Vec<i32>) -> i32 {
    vec.into_iter().fold(0, |acc, x| acc + x)
}

// currying: Transforms a function with multiple arguments into nested single-argument functions.
fn curry_add(a: i32) -> impl Fn(i32) -> i32 {
    move |b| a + b
}

// enum in FP: Defines algebraic data types, enabling state machines.
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

// Result enum Used for error handling in FP.
#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Pattern matching in fp Deconstructs data structures like enums to handle different cases.
#[derive(Debug)]
struct Human {
    name: String,
}

fn get_human_name(human: Option<Human>) -> String {
    match human {
        Option::Some(h) => h.name,
        Option::None => "No human found".to_string(),
    }
}

fn main() {
    // Recursion example
    println!("Factorial of 5: {}", factorial(5)); // Outputs: 120

    // Map example
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = map(numbers.clone(), |x| x * 2);
    println!("Map (doubled): {:?}", doubled); // Outputs: [2, 4, 6, 8, 10]

    // Pure function example
    println!("Square of 4: {}", square(4)); // Outputs: 16

    // Function composition example
    let cube = |x: i32| x * x * x;
    let add_one = |x: i32| x + 1;
    let result = compose(numbers.clone(), cube, add_one);
    println!("Composition (add_one then cube): {:?}", result); // Outputs: [8, 27, 64, 125, 216]

    // Sum example
    println!("Sum: {}", sum(numbers.clone())); // Outputs: 15

    // Currying example
    let add_five = curry_add(5);
    println!("Curried add(5, 3): {}", add_five(3)); // Outputs: 8

    // Enum example
    let opt: Option<i32> = Option::Some(42);
    println!("Enum (Option): {:?}", opt); // Outputs: Some(42)

    // Result example
    let res: Result<i32, &str> = Result::Ok(42);
    println!("Result: {:?}", res); // Outputs: Ok(42)

    // Pattern matching example
    let human = Option::Some(Human { name: "Sura".to_string() });
    println!("Pattern matching (name): {}", get_human_name(human)); // Outputs: Sura
}
