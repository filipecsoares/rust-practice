#[derive(Debug, PartialEq)]
pub struct Guitar {
    // Lots of fields here
    name: String,
    model: String,
    brand: String,
    price: f32,
}

impl Guitar {
    // This method creates a new guitar builder
    pub fn builder() -> GuitarBuilder {
        GuitarBuilder::default()
    }
}

#[derive(Default)]
pub struct GuitarBuilder {
    // Probably lots of optional fields here
    name: String,
    model: String,
    brand: String,
    price: f32,
}

impl GuitarBuilder {
    // If we have required fields, we can pass at the constructer
    pub fn new() -> GuitarBuilder {
        GuitarBuilder {
            name: String::new(),
            model: String::new(),
            brand: String::new(),
            price: 0.0,
        }
    }

    pub fn name(mut self, name: String) -> GuitarBuilder {
        // Set the name on the builder itself, and return the builder
        self.name = name;
        self
    }

    pub fn model(mut self, model: String) -> GuitarBuilder {
        self.model = model;
        self
    }

    pub fn brand(mut self, brand: String) -> GuitarBuilder {
        self.brand = brand;
        self
    }

    pub fn price(mut self, price: f32) -> GuitarBuilder {
        self.price = price;
        self
    }

    pub fn build(self) -> Guitar {
        // Create a Guitar from the GuitarBuilder
        Guitar {
            name: self.name,
            model: self.model,
            brand: self.brand,
            price: self.price,
        }
    }
}

#[test]
fn builder_test() {
    let guitar = Guitar {
        name: String::from("Gibson L-5 CES"),
        model: String::from("L-5 CES"),
        brand: String::from("Gibson"),
        price: 595.00,
    };
    let builder = GuitarBuilder::new().name(String::from("Gibson L-5 CES")).model(String::from("L-5 CES")).brand(String::from("Gibson")).price(595.00).build();
    assert_eq!(guitar, builder);
}