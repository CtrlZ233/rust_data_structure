use jemallocator;
use jemalloc_ctl::{AsName, Access};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
mod linearTable;
mod limitedLinearTable;
mod test;

use linearTable::{LinearTable, vector::Vector, list::List};

use crate::{limitedLinearTable::{stack::Stack, queue::Queue}, linearTable::list::DLinkList};
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
    {
        let mut q = List::<i32>::new();
        let mut line = String::new();
        let mut index = 100000000;
        
        while index > 0{
            q.insert(index, 0);
            index -= 1;
        }
        println!("push complete");
        
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        while !q.empty() {
            q.delete(0);
            // println!("{}", q.pop().unwrap());
        }
        println!("pop complete");
    }

    {
        let mut q = List::<i32>::new();
        let mut line = String::new();
        let mut index = 100000000;
        
        while index > 0{
            q.insert(index, 0);
            index -= 1;
        }
        println!("push complete");
        
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        while !q.empty() {
            q.delete(0);
            // println!("{}", q.pop().unwrap());
        }
        println!("pop complete");
    }

    while true {

    }

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

fn queue_test() {
    {
        let mut q = Queue::<i32>::new();
        let mut line = String::new();
        let mut index = 100000000;
        
        while index > 0{
            q.push(index);
            index -= 1;
        }
        println!("push complete");
        
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        while !q.empty() {
            q.pop().unwrap();
            // println!("{}", q.pop().unwrap());
        }
        println!("pop complete");
    }

    {
        let mut q = Queue::<i32>::new();
        let mut line = String::new();
        let mut index = 100000000;
        
        while index > 0{
            q.push(index);
            index -= 1;
        }
        println!("push complete");
        
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        while !q.empty() {
            q.pop().unwrap();
            // println!("{}", q.pop().unwrap());
        }
        println!("pop complete");
    }

    {
        let mut q = Queue::<i32>::new();
        let mut line = String::new();
        let mut index = 100000000;
        
        while index > 0{
            q.push(index);
            index -= 1;
        }
        println!("push complete");
        
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        while !q.empty() {
            q.pop().unwrap();
            // println!("{}", q.pop().unwrap());
        }
        println!("pop complete");
    }

    {
        let mut q = Queue::<i32>::new();
        let mut line = String::new();
        let mut index = 100000000;
        
        while index > 0{
            q.push(index);
            index -= 1;
        }
        println!("push complete");
        
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        while !q.empty() {
            q.pop().unwrap();
            // println!("{}", q.pop().unwrap());
        }
        println!("pop complete");
    }
    
    while true {
        
    }
}

fn main() {
    // vector_test();
    // list_test();
    // stack_test();
    // let mut a: usize = 0;
    // a -= 1;
    // println!("{}", a);
    // test();
    queue_test();
    
}