use std::collections::HashSet;

fn main() {

    let north_american_cats_set: HashSet<_> = ["Puma", "Canada Lynx", "Bobcat"].iter().cloned().collect();
    let north_american_cats_vec: Vec<_> = ["Puma", "Canada Lynx", "Bobcat"].iter().cloned().collect();

    for cat in &north_american_cats_set {
        println!("set cat ref: {}", cat);
    }

    for cat in &north_american_cats_vec {
        println!("vec cat ref: {}", cat);
    }

//    for cat in &mut north_american_cats_set {
//        println!("set cat mut ref: {}", cat);
//    }

//    for cat in &mut north_american_cats_vec {
//        println!("vec cat mut ref: {}", cat);
//    }

    for cat in north_american_cats_set {
        println!("set ownership cat {}", cat);
    }

    for cat in north_american_cats_vec {
        println!("vec ownership cat {}", cat);
    }
}

