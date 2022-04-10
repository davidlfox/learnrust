pub struct Item {
    pub name: String,
}

impl Item {
    pub fn build(name: String) -> Item {
        Item {
            name
        }
    }
}