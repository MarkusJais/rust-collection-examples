#[derive(Debug, Eq, PartialEq)]
struct Person {
    name: String,
    height_in_cm: i32,
    age: i32,
}


fn main() {

    let mut people: Vec<Person> = Vec::new();
    people.push(Person{name: "John".to_string(), height_in_cm: 180, age: 60});
    people.push(Person{name: "Carmen".to_string(), height_in_cm: 160, age: 30});
    people.push(Person{name: "Maria".to_string(), height_in_cm: 171, age: 25});

    let more_than_170 = people.iter().
        filter( | person | person.height_in_cm > 170).
        collect::<Vec<&Person>>();

    println!("more than 170:{:?}", more_than_170);
}




