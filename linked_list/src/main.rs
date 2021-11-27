mod linked_list;

use linked_list::LinkedList;

fn main() {
    let mut mylist = LinkedList::from(vec![1, 2, 3, 4]);
    mylist.push_back(5);
    assert_eq!(mylist.pop_front(), Some(1));
    assert_eq!(mylist.pop_front(), Some(2));
    assert_eq!(mylist.pop_front(), Some(3));
    mylist.push_front(6);
    assert_eq!(mylist.pop_front(), Some(6));
    assert_eq!(mylist.pop_front(), Some(4));
    assert_eq!(mylist.pop_front(), Some(5));
    assert_eq!(mylist.pop_front(), None);
    assert_eq!(mylist.pop_front(), None);

    let mut list1 = LinkedList::from(vec![1, 2, 3, 4]);
    let mut list2 = LinkedList::from(vec![5, 6, 7, 8]);
    list1.append_back(&mut list2);
    assert_eq!(list2.is_empty(), true);
    assert_eq!(list1.pop_all(), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(list1.is_empty(), true);
}
