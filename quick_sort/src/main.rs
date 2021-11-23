// Implemeting quick sort

mod quick_sort;

fn main() {
    let mut arr = vec![1, 2, 3, 4];
    quick_sort::sort(& mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4]);

    let mut arr = vec![4, 3, 2, 1];
    quick_sort::sort(& mut arr);
    assert_eq!(arr, vec![1, 2, 3, 4]);

    let mut arr = vec![2, 5, 3, -1, 6, 2, 3, 10, 1, 6];
    quick_sort::sort(& mut arr);
    assert_eq!(arr, vec![-1, 1, 2, 2, 3, 3, 5, 6, 6, 10]);

    let mut arr = vec!["ASDF", "ABC", "dfkjsfkl", "asajdjs"];
    quick_sort::sort(& mut arr);
    assert_eq!(arr, vec!["ABC", "ASDF", "asajdjs", "dfkjsfkl"]);

    let mut arr = vec![1, 2, 3, 4];
    quick_sort::sort_by(& mut arr, & |a, b| b.partial_cmp(a).unwrap());
    assert_eq!(arr, vec![4, 3, 2, 1]);

    let mut arr = vec!["ASDF", "ABC", "dfkjsfkl", "asajdjs"];
    quick_sort::sort_by(& mut arr, & |a, b| a.to_lowercase().partial_cmp(&b.to_lowercase()).unwrap());
    assert_eq!(arr, vec!["ABC", "asajdjs", "ASDF", "dfkjsfkl"]);
}
