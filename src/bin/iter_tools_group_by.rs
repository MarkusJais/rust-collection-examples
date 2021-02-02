extern crate itertools;

use itertools::Itertools;

fn main() {


// group data into runs of larger than zero or not.
    let data = vec![1, 3, -2, -2, 1, 0, 1, 2];
// groups:     |---->|------>|--------->|

// Note: The `&` is significant here, `GroupBy` is iterable
// only by reference. You can also call `.into_iter()` explicitly.

    let mut data_grouped = Vec::new();
    for (key, group) in &data.into_iter().group_by(|elt| *elt >= 0) {
        data_grouped.push((key, group.collect()));
    }

    for k in &data_grouped {
        println!("{:?}", k)
    }

    assert_eq!(data_grouped, vec![(true, vec![1, 3]), (false, vec![-2, -2]), (true, vec![1, 0, 1, 2])]);

}