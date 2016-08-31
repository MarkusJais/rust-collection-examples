
fn main() {
    let num_array = [1, 222, 3, 33, 1000];
    let array_iter = num_array.iter().filter( |&&x| x > 100);
    let values = array_iter.collect::<Vec<_>>();
    println!("values:{:?}", values);
}
// for good explanation see: http://stackoverflow.com/questions/30467085/how-to-iterate-over-an-array
