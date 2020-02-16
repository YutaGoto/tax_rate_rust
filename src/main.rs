#[derive(Debug)]
pub struct Item {
    name: String,
    item_type: String,
    is_reduce_tax: bool,
}

#[derive(Debug)]
pub struct Product {
    name: String,
    item_type: String,
    price: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, item_type: impl Into<String>, is_reduce_tax: bool) -> Item {
        Item {
            name: name.into(),
            item_type: item_type.into(),
            is_reduce_tax: is_reduce_tax,
        }
    }
}

const fn create_item_list() -> [Item; 5] {
    return [
        Item::new("食料品", "food", true),
        Item::new("飲料品", "beverage", true),
        Item::new("酒類", "liquor", true),
        Item::new("医薬品", "drug", true),
        Item::new("医薬部外品", "quasi_drug", true),
    ]
}

static ITEM_LIST: &'static [Item; 5] = &create_item_list();

fn judge_reduced_tax(product: Product) -> bool {
    for item in ITEM_LIST {
        if item.item_type == product.item_type {
            return item.is_reduce_tax
        }
    }
    return false;
}

fn main() {
    println!("Hello, world!");
    let product = Product {name: "手巻直火焼き紅しゃけ".to_string(), item_type: "food".to_string(), price: 130};
    println!("{}", judge_reduced_tax(product));
}
