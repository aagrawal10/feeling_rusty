// Given a list of flattened json fields, return the JSON tree for that list
// e.g. ["a.b", "a.b.c", "a", "c.d"] should return
// [a->a.b->a.b.c, c->c.d]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;


#[derive(Debug)]
struct Column {
    name: String,
    children: Rc<RefCell<Vec<Rc<Column>>>>,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct ResultColumn {
    name: String,
    children: Vec<ResultColumn>,
}

impl ResultColumn {
    fn from(column: Column) -> ResultColumn {
        ResultColumn {
            name: column.name,
            children: Rc::try_unwrap(column.children).unwrap().into_inner().into_iter().map(
                |x| ResultColumn::from(Rc::try_unwrap(x).unwrap())
            ).collect(),
        }
    }

    fn pretty_print(& self, indent_level: usize) {
        println!("{}{}", String::from_iter(vec!["-"; indent_level]) ,self.name);
        for child in self.children.iter() {
            (*child).pretty_print(indent_level + 1);
        }
    }
}

fn get_json_tree(flattened_list: & mut Vec<& str>) -> Vec<ResultColumn> {
    let json_tree: Rc<RefCell<Vec<Rc<Column>>>> = Rc::new(RefCell::new(Vec::new()));
    let mut json_tree_map: HashMap<String, Rc<Column>> = HashMap::new();

    flattened_list.sort_by_cached_key(|item| item.split('.').count());
    for item in flattened_list {
        let tokens: Vec<& str> = item.split('.').collect();
        let mut parent_list = Rc::clone(&json_tree);
        for (index, _) in tokens.iter().enumerate() {
            let curr_full_name = tokens[0..index + 1].join(".");
            if json_tree_map.contains_key(&curr_full_name as &str) {
                // The current subnode already exists in the tree
                // Any child nodes should be added to children list of this node
                let existing_column = json_tree_map.get(&curr_full_name as &str).unwrap();
                parent_list = Rc::clone(&existing_column.children);
                continue;
            }

            // The current subnode doesn't exist in the tree.
            // Ideally, code should only reach here for curr_full_name == item
            // However, in cases where a field "a.b" is included in the field_list, but field "a"
            // is not included, code can get here for curr_full_name != item. Handle that case
            // by adding a subnode with name and label equal to curr_full_name
            let new_col = Rc::new(
                Column {
                    name: String::from(&curr_full_name),
                    children: Rc::new(RefCell::new(Vec::new())),
                }
            );
            (*parent_list).borrow_mut().push(Rc::clone(& new_col));
            parent_list = Rc::clone(&new_col.children);
            json_tree_map.insert(String::from(&curr_full_name), Rc::clone(& new_col));
        }
    }

    drop(json_tree_map);

    return Rc::try_unwrap(json_tree).unwrap().into_inner().into_iter().map(
        |x| ResultColumn::from(Rc::try_unwrap(x).unwrap())
    ).collect();
}


fn main() {
    let mut simple_list = vec!["a.b", "a.b.c", "a", "c.d"];
    let json_tree = get_json_tree(& mut simple_list);
    println!("{:?}", simple_list);
    for item in json_tree.iter() {
        item.pretty_print(0);
    }
    assert_eq!(
        json_tree,
        vec! [
            ResultColumn{
                name: String::from("a"),
                children: vec![
                    ResultColumn {
                        name: String::from("a.b"),
                        children: vec! [
                            ResultColumn {
                                name: String::from("a.b.c"),
                                children: vec! [],
                            }
                        ]
                    },
                ]
            },
            ResultColumn {
                name: String::from("c"),
                children: vec! [
                    ResultColumn {
                        name: String::from("c.d"),
                        children: vec! [],
                    }
                ]
            }
        ]
    );

    let mut more_complex_list = vec![
        "name", "address", "address.city", "address.zipcode", "age", "card", "card.last4", "card.expiry", "card.issue",
        "dob", "dob.year", "dob.month", "dob.day", "temp1.temp2.temp3.temp4"
    ];
    let json_tree = get_json_tree(& mut more_complex_list);
    println!("{:?}", more_complex_list);
    for item in json_tree.iter() {
        item.pretty_print(0);
    }
}
