#[derive(Debug)]
enum HealthStatus {
    Stable,
    IntensiveCare,
    Critical,
}

#[derive(Debug)]
struct Patient {
    name: String,
    id: u32,
    health: HealthStatus,
}

impl Patient {
    fn new(name: String, id: u32, health: HealthStatus) -> Self {
        Self { name, id, health }
    }
}

// Define the Node struct that represents a single element in the list
struct Node {
    value: Patient,
    next: Option<Box<Node>>,
}

// Define the LinkedList struct that manages the list
struct LinkedList {
    head: Option<Box<Node>>,
    tail: Option<*mut Node>,
}

impl LinkedList {
    // create an empty LinkedList
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
    // check if the LinkedList is empty
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    // push a value to the front of the LinkedList
    fn push(&mut self, value: Patient) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        let raw_node = Box::into_raw(new_node);
        self.head = unsafe { Some(Box::from_raw(raw_node)) };
        if self.tail.is_none() {
            self.tail = Some(raw_node);
        }
    }
    // Pop the value from the front of the LinkedList
    fn pop(&mut self) -> Option<Patient> {
        self.head.take().map(|node| {
            self.head = node.next;

            if self.head.is_none() {
                self.tail = None;
            }

            node.value
        })
    }
    // Print all items in the Linked List
    fn print_all(&self) {
        let mut current_node = &self.head;
        while let Some(node) = current_node {
            println!(
                "{:?} {:?} {:?}",
                node.value.name, node.value.id, node.value.health
            );
            current_node = &node.next;
        }
    }
}

// Example usage
fn main() {
    let mut list: LinkedList = LinkedList::new();

    list.push(Patient::new(
        "Filipe".to_string(),
        77,
        HealthStatus::Critical,
    ));
    list.push(Patient::new("Mary".to_string(), 88, HealthStatus::Stable));
    list.push(Patient::new(
        "Test".to_string(),
        99,
        HealthStatus::IntensiveCare,
    ));

    list.print_all();
    list.pop();
    if !list.is_empty() {
        list.print_all();
    }
}
