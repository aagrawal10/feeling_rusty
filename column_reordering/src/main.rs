// WAP to get the right order of columns given the following
// Context: While re-importing data from CSV to G-Sheet, we need to preserve the order
// of columns selected by the user. However, there might be new columns in the CSV as well
// The best effort logic is to append that to the end. Further, there can be duplicate column
// names as well. Hence, we need to compute the index order of source_columns representing
// the order in which they should be written to the sheet
// Params:
//   curr_columns: Vec<str> (Columns can be duplicate) - This is the current set of columns in G-Sheet
//   source_columns: Vec<str> (Columns can be duplicate) - This is the new set of columns in CSV
// Output:
//   result: Vec<i32> where output[i] is the index of column in source_columns
// Final column order should be such that all curr_columns should appear in the current order
// if they are in source_columns, The rest of the columns in source_columns should appear in order

use std::collections::HashMap;


fn get_final_column_order(curr_columns: Vec<& str>, source_columns: Vec<& str>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    // Create a map of column_name by count in curr_columns
    let mut num_columns_by_name: HashMap<&str, i32> = HashMap::new();
    for column_name in source_columns.iter() {
        let curr_value = num_columns_by_name.entry(column_name).or_insert(0);
        *curr_value += 1;
    }

    let mut num_columns_in_header_by_name: HashMap<&str, i32> = HashMap::new();
    for column_name in curr_columns.iter() {
        if !num_columns_by_name.contains_key(column_name) {
            // The column is no longer present in the source, skip this.
            continue;
        }

        let num_columns_already_in_header = *num_columns_in_header_by_name.entry(column_name).or_insert(0);
        if num_columns_already_in_header >= num_columns_by_name[column_name] {
            // We have already included max columns with this name, skip this column
            continue;
        }

        let all_indices_of_column: Vec<usize> = source_columns.iter()
                                                            .enumerate()
                                                            .filter(|&(_, cn)| cn == column_name)
                                                            .map(|(i, _)| i)
                                                            .collect();
        result.push(all_indices_of_column[num_columns_already_in_header as usize] as i32);
        *num_columns_in_header_by_name.get_mut(column_name).unwrap() += 1;
    }

    // Now add indices of columns in source_columns which are not in result
    for (index, column_name) in source_columns.iter().enumerate() {
        if result.contains(&(index as i32)) || column_name.is_empty(){
            // Column index already in result, skip this.
            continue;
        }

        result.push(index as i32);
    }

    return result;
}

fn main() {
    let curr_columns: Vec<&str> = vec![
        "A", "B", "C", "D", "E", "F", "F", "G", "G", "H"
    ];

    let source_columns: Vec<&str> = vec![
        "Z", "A", "B", "B", "C", "F", "G", "H", "H"
    ];

    let result = get_final_column_order(curr_columns, source_columns);
    println!("{:?}", result);

    // Final set of columns should be
    // A B C F G H Z B H
    assert_eq!(result, vec![1, 2, 4, 5, 6, 7, 0, 3, 8])
}
