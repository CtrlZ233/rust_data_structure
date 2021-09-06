mod linearTable;
mod limitedLinearTable;
mod test;
use std::ops::Index;

use linearTable::{LinearTable, vector::Vector, list::List};

use crate::limitedLinearTable::stack::Stack;
fn vector_test() {
    println!("======================== vector test ===========================");
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
    // c.merge(a);
    c.traverse(print);
}   

fn list_test() {
    println!("======================== list test ===========================");
    let mut l1 = List::<i32>::new();
    let print = |x: &i32| {
        println!("{}", x);
    };
    l1.insert(1, 0);
    l1.insert(2, 1);
    l1.insert(4, 2);
    l1.delete(1).unwrap();
    // println!("{}", l.next_elem(4).unwrap());
    l1.traverse(print);

    let mut l2 = List::<i32>::new();
    l2.insert(3, 0);
    l2.insert(5, 1);
    // l1.merge(l2);
    l1.traverse(print);
}

fn stack_test() {
    println!("======================== stack test ===========================");
    let mut s1 = Stack::<i32>::new();
    let print = |x: &i32| {
        println!("{}", x);
    };
    s1.push(1);
    s1.push(2);
    s1.push(3);
    s1.pop();
    s1.push(4);
    s1.push(5);
    s1.pop();
    s1.push(6);
    s1.traverse(print);

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
    list_test();
    stack_test();
    // let mut a: usize = 0;
    // a -= 1;
    // println!("{}", a);
    // test();
}