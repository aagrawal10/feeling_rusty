// Quick Sort implementation using generics
// TODO: Add support for custom cmp operator

fn partition<T: PartialOrd> (arr: & mut Vec<T>, low: usize, high: usize) -> usize {
    let mut i = low;
    let mut j = high;

    while i < j {
        if arr[i] <= arr[high] {
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

fn sort_internal<T: PartialOrd> (arr: & mut Vec<T>, low: usize, high: usize) {
    if low >= high {
        return;
    }

    let partition_index = partition(arr, low, high);
    if partition_index > low {
        sort_internal(arr, low, partition_index - 1);
    }

    if partition_index < high {
        sort_internal(arr, partition_index + 1, high);
    }
}

pub fn sort<T: PartialOrd> (arr: & mut Vec<T>) {
    sort_internal(arr, 0, arr.len() - 1);
}
