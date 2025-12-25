use std::{error::Error, fs::read_to_string};

fn main() {
    println!("Hello, world!");
    let num = 42;
    println!("{}", num.summarize());
    notify(&num);

    // Handling the Option type with pattern matching
    let name: Option<String> = Some(String::from("Alice"));
    match name {
        Some(ref n) if n.len() > 3 => println!("Long name: {}", n),
        Some(ref n) => println!("Short name: {}", n),
        None => println!("No name provided"),
    }

    // Handling the Result type with pattern matching
    let result = read_to_string("src/text.txt");
    match result {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    // Using the function that returns Result
    match get_adult_user_names("src/users.csv") {
        Ok(names) => {
            for name in names {
                println!("Adult user: {}", name);
            }
        }
        Err(e) => println!("Error: {}", e),
    }

    // Using the enum and handling different variants
    let request = WebRequest::Success(String::from("Data loaded successfully"));
    handle_request(request);
}

// Reference with lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Trait with a default method implementation
fn notify<T: Summary>(item: &T) {
    println!("Notification: {}", item.summarize());
}

// Trait with a method
trait Summary {
    fn summarize(&self) -> String;
}

// Even a primitive type can implement a trait
impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("This is the number: {}", self)
    }
}

// Function that showcases handling the Result type
fn get_adult_user_names(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let content = read_to_string(file_path)?;

    let names = content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let name: &&str = parts.first()?;
            let age: i32 = parts.get(1)?.parse().ok()?;
            if age >= 20 {
                Some(name.to_string())
            } else {
                None
            }
        })
        .collect();
    Ok(names)
}


// Enum with various variants
enum WebRequest {
    Loading,
    Success(String),
    Failure { code: i32, message: String },
    Timeout,
    TooManyRequests { retry_after: u32 },
}

// Enum can even have methods
impl WebRequest {
    fn is_finished(&self) -> bool {
        match self {
            matches!(self, WebRequest::Loading) => false,
        }
    }
}

// Enum pattern matching example
fn handle_request(request: WebRequest) {
    println!("Request is finished: {}", request.is_finished());
    match request {
        WebRequest::Loading => println!("Request is loading..."),
        WebRequest::Success(data) => println!("Request succeeded with data: {}", data),
        WebRequest::Failure { code, message } => {
            println!("Request failed with code {}: {}", code, message)
        }
        WebRequest::Timeout => println!("Request timed out."),
        WebRequest::TooManyRequests { retry_after } => {
            println!("Too many requests. Retry after {} seconds.", retry_after)
        }
    }
}
