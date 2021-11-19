// Imagine a single lane freeway (no overtaking allowed) where cars are 
// moving in uniform speed. Given the speed of each car WAF to find the number
// of cars in each cluster.
// -----------------------------------
// 
// [2, 4, 1, 3] -> [2, 2]
// [2, 5, 4, 3, 1] -> [4, 1]
// -----------------------------------
//
fn get_length_of_clusters(list_of_speeds: &Vec<i32>) -> Vec<i32> {
    if list_of_speeds.len() == 0 {
        return Vec::new();
    }

    let mut result: Vec<i32> = Vec::new();
    let mut min_speed = list_of_speeds[0];
    let mut curr_cluster_size = 1;
    for index in 1..list_of_speeds.len() {
        if list_of_speeds[index] > min_speed {
            curr_cluster_size += 1;
            continue;
        }

        result.push(curr_cluster_size);
        curr_cluster_size = 1;
        min_speed = list_of_speeds[index];
    }

    result.push(curr_cluster_size);
    return result;
}


// A fastest car is inserted at all possible indices on the original freeway. 
// For each of the possible permutations compute the number of cars in each
// cluster
// [2, 4, 1, 3] -> [[1, 2, 2], [3, 2], [3, 2], [2, 3], [2, 3]]
// Constraint: You can call the first function only once.
 fn get_clusters_with_fastest_car(list_of_speeds: &Vec<i32>) -> Vec<Vec<i32>> {
     let mut result: Vec<Vec<i32>> = Vec::new();
     let length_of_clusters = get_length_of_clusters(&list_of_speeds);

     // The first item in result is always [1, length_of_clusters]
     let mut first_item = length_of_clusters.clone();
     first_item.insert(0, 1);
     result.push(first_item);

     for (index, item) in length_of_clusters.iter().enumerate() {
         for _ in 0..*item {
             let mut curr_list = length_of_clusters.clone();
             curr_list[index] = *item + 1;
             result.push(curr_list);
         }
     }

     return result;
 }


fn main() {
    // Testing the first function
    assert_eq!(get_length_of_clusters(&vec![2, 4, 1, 3]), vec![2, 2]);
    assert_eq!(get_length_of_clusters(&vec![2, 5, 4, 3, 1]), vec![4, 1]);
    assert_eq!(get_length_of_clusters(&vec![2]), vec![1]);
    assert_eq!(get_length_of_clusters(&vec![]), vec![]);

    // Testing the second function
    assert_eq!(
        get_clusters_with_fastest_car(&vec![2, 4, 1, 3]),
         vec![
             vec![1, 2, 2],
             vec![3, 2],
             vec![3, 2],
             vec![2, 3],
             vec![2, 3],
         ]
    );
    assert_eq!(
        get_clusters_with_fastest_car(&vec![2, 5, 4, 3, 1]),
         vec![
             vec![1, 4, 1],
             vec![5, 1],
             vec![5, 1],
             vec![5, 1],
             vec![5, 1],
             vec![4, 2],
         ]
    );
    assert_eq!(
        get_clusters_with_fastest_car(&vec![2]),
         vec![
             vec![1, 1],
             vec![2],
         ]
    );
    assert_eq!(
        get_clusters_with_fastest_car(&vec![]),
         vec![vec![1]]
    );

}
