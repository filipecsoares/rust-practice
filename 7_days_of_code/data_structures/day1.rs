struct Item {
    name: String,
    quantity: u32,
}

struct ShoppingList {
    items: Vec<Item>,
}

impl ShoppingList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add_item(&mut self, name: String, quantity: u32) {
        let item = Item { name, quantity };
        self.items.push(item);
    }

    fn remove_item(&mut self, name: &str) {
        self.items.retain(|item| item.name != name);
    }

    fn list_items(&self) {
        for item in &self.items {
            println!("{} - {}", item.name, item.quantity);
        }
    }
}

fn main() {
    let mut shopping_list = ShoppingList::new();
    shopping_list.add_item("Apple".to_string(), 5);
    shopping_list.add_item("Guitar".to_string(), 2);
    shopping_list.add_item("Melon".to_string(), 3);
    shopping_list.add_item("Orange".to_string(), 6);

    shopping_list.list_items();
    shopping_list.remove_item("Melon");
    shopping_list.list_items();
}
