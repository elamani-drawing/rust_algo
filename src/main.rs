use rust_algo::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.add(5);
    list.add(6);
    assert_eq!(list.index_of(6), 1);
    dbg!(list);
}
 