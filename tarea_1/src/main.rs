
use std::io;
use std::io::Write;
use std::process;

struct Node {
    data: f64,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Option<Box<Node>>,
}

// Linked list implementation
impl LinkedList {
    // Create a new linked list
    fn new() -> LinkedList {
        LinkedList { head: None }
    }

    // Add a node to the linked list
    fn add(&mut self, data: f64) {
        let new_node = Node {
            data: data,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }

    // Calculate the mean of the linked list
    fn mean(&self) -> f64 {
        let mut sum = 0.0;
        let mut count = 0.0;
        let mut current = &self.head;
        while let Some(ref node) = *current {
            sum += node.data;
            count += 1.0;
            current = &node.next;
        }
        sum / count
    }

    // Calculate the standard deviation of the linked list
    fn standard_derivation(&self) -> f64 {
        let mean = self.mean();
        let mut sum = 0.0;
        let mut count = 0.0;
        let mut current = &self.head;
        while let Some(ref node) = *current {
            sum += (node.data - mean).powi(2);
            count += 1.0;
            current = &node.next;
        }
        (sum / (count - 1.0)).sqrt()
    }
}

fn main() {
    let mut list = LinkedList::new();
    let mut input = String::new();
    println!("Enter a number or 'q' to quit");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if input.trim() == "q" {
            break;
        }
        let num: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number or 'q' to quit");
                continue;
            }
        };
        list.add(num);
        input.clear();
    }
    println!("Mean: {}", list.mean());
    println!("Standard Deviation: {}", list.standard_derivation());
    process::exit(0);
}
