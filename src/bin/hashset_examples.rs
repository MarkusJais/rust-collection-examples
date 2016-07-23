use std::collections::HashSet;

fn main() {
    let mut asian_cats = HashSet::new();
    asian_cats.insert("Tiger");
    asian_cats.insert("Leopard");
    asian_cats.insert("Snow Leopard");
    asian_cats.insert("Cheetah");

    let african_cats: HashSet<_> = ["Lion", "Lion", "Leopard", "Cheetah", "Serval"].iter().cloned().collect();
    println!("asian cats:{:?}", asian_cats);
    println!("african cats:{:?}", african_cats);
    let cats_on_both_continents: HashSet<_> = asian_cats.intersection(&african_cats).cloned().collect();

    println!("african AND asian cats:{:?}", cats_on_both_continents);

}

