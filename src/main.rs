fn bubble_sort<T: PartialOrd + Copy>(list: &mut Vec<T>) -> &Vec<T> {
    for i in 0..list.len() {
        for x in 0..list.len() -1-i {
            if list[x] > list[x + 1] {
                list.swap(x, x + 1);
            }
        }
    }
    list
}
fn main() {

    let mut list = vec![1, 6, 2555, 24, 35, 27, 44, 2, 432];
    bubble_sort(&mut list);
    println!("{:?}  ", list);

    let mut list = vec!['g', 'h', 'T', 'R', 'b', 'f'];
    bubble_sort(&mut list);
    println!("{:?}  ", list);
}