mod linearTable;

use linearTable::{LinearTable, vector::Vector};
fn vector_test() {
    let mut c = Vector::<i32>::new();
    c.insert(1, 0);
    c.insert(2, 1);
    c.insert(4, 2);
    let print = |x: &i32| {
        println!("{}", x);
    };
    c.delete(0).unwrap();
    // println!("{}", c.get_elem(1).unwrap());
    c.traverse(print);

    let mut a = Vector::<i32>::new();
    a.insert(3, 0);
    a.insert(5, 1);
    a.insert(6, 2);
    c.merge(a);
    c.traverse(print);
}   

#[derive(Debug)]
struct t {
    elem: i32,
}

// fn test() {
//     let c = Some(t{elem: 5});
//     match c {
//         Some(t) => {
//             println!("dsd");
//         }
//         _ => {}
//     }

//     println!("{:?}", c.unwrap());
// }


fn main() {
    vector_test();
    // test();
}