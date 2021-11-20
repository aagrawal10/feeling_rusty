// Quick Sort implementation using generics

fn partition<T, F: Fn(&T, &T) -> bool> (
    arr: & mut Vec<T>,
    low: usize,
    high: usize,
    cmp: & F,
) -> usize {
    let mut i = low;
    let mut j = high;

    while i < j {
        if !cmp(&arr[i], &arr[high]) {
            i += 1;
            continue;
        }

        j -= 1;
        arr.swap(i, j);
    }

    assert_eq!(i, j);
    arr.swap(i, high);
    return i;
}

fn sort_internal<T, F> (
    arr: & mut Vec<T>,
    low: usize,
    high: usize,
    cmp: & F
) where F: Fn(&T, &T) -> bool{
    if low >= high {
        return;
    }

    let partition_index = partition(arr, low, high, cmp);
    if partition_index > low {
        sort_internal(arr, low, partition_index - 1, cmp);
    }

    if partition_index < high {
        sort_internal(arr, partition_index + 1, high, cmp);
    }
}

pub fn sort<T: PartialOrd> (arr: & mut Vec<T>) {
    sort_internal(arr, 0, arr.len() - 1, & |a: &T, b: &T| {
        match a.partial_cmp(b).unwrap() {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Greater => true,
        }
    });
}

pub fn sort_by<T, F> (
    arr: & mut Vec<T>,
    cmp: & F
) where F: Fn(&T, &T) -> std::cmp::Ordering{
    sort_internal(arr, 0, arr.len() - 1, & |a: &T, b: &T| {
        match cmp(a, b) {
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Greater => true,
        }
    });
}
