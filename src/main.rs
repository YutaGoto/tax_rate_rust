#[derive(Debug)]
pub struct Item {
    name: String,
    item_type: String,
    is_reduce_tax: bool,
}

#[derive(Debug, Clone)]
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
            is_reduce_tax: is_reduce_tax.into(),
        }
    }
}

fn judge_reduced_tax(product: &Product) -> bool {
    let item_list = [
        Item::new("食料品", "food", true),
        Item::new("飲料品", "beverage", true),
        Item::new("酒類", "liquor", false),
        Item::new("医薬品", "drug", false),
        Item::new("医薬部外品", "quasi_drug", false),
    ];
    for item_num in 0..item_list.len() {
        if item_list[item_num].item_type == product.item_type {
            return item_list[item_num].is_reduce_tax
        }
    }
    return false;
}

fn main() {
    let product_list: [Product; 5] = [
        Product {name: "手巻直火焼き紅しゃけ".to_string(), item_type: "food".to_string(), price: 130},
        Product {name: "キリン 生茶 555ml ペットボトル".to_string(), item_type: "beverage".to_string(), price: 140},
        Product {name: "キリンチューハイ氷結グレープフルーツ350ml缶".to_string(), item_type: "liquor".to_string(), price: 141},
        Product {name: "新ルルＡ錠ｓ 50錠".to_string(), item_type: "drug".to_string(), price: 871},
        Product {name: "リポビタン".to_string(), item_type: "quasi_drug".to_string(), price: 146},
    ];
    for product_num in 0..product_list.len() {
        println!("{}", judge_reduced_tax(&product_list[product_num]));
    }
}
