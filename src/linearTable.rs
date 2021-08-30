
pub trait LinearTable<T>
    where T: PartialEq + PartialOrd + Clone
{
    fn destroy(&mut self);
    fn clear(&mut self);
    fn empty(&self) -> bool;
    fn length(&self) -> usize;
    fn get_elem(&self, index: usize) -> Option<T>;
    fn locate_elem(&self, elem: T) -> Option<usize>;
    fn prior_elem(&self, elem: T) -> Option<T>;
    fn next_elem(&self, elem: T) -> Option<T>;
    fn insert(&mut self, elem: T, index: usize) -> CommResult;
    fn delete(&mut self, index: usize) -> Result<T, ()>;
    fn traverse<F: Fn(&T)>(&self, func: F);
    fn merge(&mut self, other: Self);
}

pub enum CommResult {
    Ok,
    Err,
}

pub mod vector {

    use super::{CommResult, LinearTable};

    pub struct Vector<T>
        where T: PartialEq + PartialOrd + Clone
    {
        vector: Vec<T>,
    }

    impl<T> Vector<T>
        where T: PartialEq + PartialOrd + Clone
    {
        pub fn new() -> Self {
            Vector { vector: Vec::<T>::new() }
        }
    }
    impl<T> LinearTable<T> for Vector<T>
        where T: PartialEq + PartialOrd + Clone
    {

        fn destroy(&mut self) {
            // do nothing
        }

        fn clear(&mut self) {
            self.vector.clear();
        }

        fn empty(&self) -> bool {
            self.vector.is_empty()
        }

        fn length(&self) -> usize {
            self.vector.len()
        }

        fn get_elem(&self, index: usize) -> Option<T> {
            if index >= self.length() {
                None
            } else {
                Some(self.vector[index].clone())
            }
        }

        fn locate_elem(&self, elem: T) -> Option<usize> {
            for (i, v) in self.vector.iter().enumerate() {
                if *v == elem {
                    return Some(i)
                }
            }
            None
        }

        fn prior_elem(&self, elem: T) -> Option<T> {
            if let Some(index) = self.locate_elem(elem) {
                if index == 0 {
                    None
                } else {
                    Some(self.vector[index - 1].clone())
                }
            } else {
                None
            }
        }

        fn next_elem(&self, elem: T) -> Option<T> {
            if let Some(index) = self.locate_elem(elem) {
                if index == self.length() - 1 {
                    None
                } else {
                    Some(self.vector[index + 1].clone())
                }
            } else {
                None
            }
        }

        fn insert(&mut self, elem: T, index: usize) -> CommResult {
            if index > self.length() {
                CommResult::Err
            } else {
                self.vector.insert(index, elem);
                CommResult::Ok
            }
        }

        fn delete(&mut self, index: usize) -> Result<T, ()> {
            if index >= self.length() {
                Err(())
            } else {
                Ok(self.vector.remove(index))
            }
        }

        fn traverse<F: Fn(&T)>(&self, func: F) {
            for v in self.vector.iter() {
                func(v);
            }
        }

        fn merge(&mut self, other: Vector<T>) {
            let mut tmp = Vec::<T>::new();
            let mut iter1 = self.vector.iter();
            let mut iter2 = other.vector.iter();
            let mut a = iter1.next();
            let mut b = iter2.next();
            loop {
                match (a, b) {
                    (Some(value_a), Some(value_b)) => {
                        if value_a <= value_b {
                            tmp.push(value_a.clone());
                            a = iter1.next();
                        } else {
                            tmp.push(value_b.clone());
                            b = iter2.next();
                        }
                    }
                    _ => break
                }
            }

            while let Some(value_a) = a {
                tmp.push(value_a.to_owned());
                a = iter1.next();
            }
            while let Some(value_b) = b {
                tmp.push(value_b.to_owned());
                b = iter2.next();
            }

            self.vector = tmp;
        }
    }
}

pub mod list {

    use std::{borrow::Borrow, cell::RefCell, convert::TryInto, rc::Rc};

    use super::{CommResult, LinearTable};

    pub struct List<T>
        where T: PartialEq + PartialOrd + Clone
    {
        head: Rc<RefCell<Node<T>>>,
        len: usize,
    }

    struct Node<T>
        where T: PartialEq + PartialOrd + Clone
    {
        data: Option<T>,
        next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T> List<T>
        where T: PartialEq + PartialOrd + Clone
    {
        pub fn new() -> Self {
            List { head: Rc::new(RefCell::new(Node {data: None, next: None})), len: 0, }
        }
    }

    impl<T> LinearTable<T> for List<T>
        where T: PartialEq + PartialOrd + Clone
    {
        fn destroy(&mut self) {
            self.head.borrow_mut().next = None;
        }

        fn clear(&mut self) {
            self.head.borrow_mut().next = None;
        }

        fn empty(&self) -> bool {
            match (*self.head).borrow().next {
                None => true,
                _ => false,
            }
        }

        fn length(&self) -> usize {
            self.len
        }

        fn get_elem(&self, index: usize) -> Option<T> {
            let mut index: i32 = index.try_into().unwrap();
            let mut pre = (*self.head).borrow().next.clone();
            while index >= 0 {
                match pre {
                    Some(t) => {
                        let c = (*t).borrow().next.clone();
                        pre = c;
                        index -= 1;
                    }
                    _ => return None,
                }
            }
            pre.unwrap().as_ref().borrow().data.clone()
        }

        fn locate_elem(&self, elem: T) -> Option<usize> {
            let mut pre = self.head.as_ref().borrow().next.clone();
            let mut index = 0;
            while let Some(t) = pre {
                if t.as_ref().borrow().data.clone().unwrap() == elem {
                    return Some(index);
                }
                pre = t.as_ref().borrow().next.clone();
                index += 1;
            }
            None
        }

        fn prior_elem(&self, elem: T) -> Option<T> {
            // if self.empty() {
            //     return None;
            // }
            // let mut pre = self.next.as_ref().unwrap();
            // let mut cur =  pre.next.as_ref();
            // while let Some(t) = cur {
            //     if t.data == elem {
            //         return Some(pre.data.clone());
            //     }
            //     pre = cur.unwrap();
            //     cur = pre.next.as_ref();
            // }
            None
        }

        fn next_elem(&self, elem: T) -> Option<T> {
            // if self.empty() {
            //     return None;
            // }
            // let mut cur = self.next.as_ref().unwrap();
            // let mut next = cur.next.as_ref();
            // while let Some(t) = next {
            //     if cur.data == elem {
            //         return Some(t.data.clone());
            //     }
            //     cur = next.unwrap();
            //     next = cur.next.as_ref();
            // }
            None
        }

        fn insert(&mut self, elem: T, index: usize) -> CommResult {
            // let mut index = index;
            // if index == 0 {
            //     let newNode = Rc::new(Node{data: elem, next: self.next.clone()});
            //     self.next = Some(newNode);
            //     return CommResult::Ok;
            // }
            // let mut pre = self.next.as_ref();
            // while index > 1 {
            //     match pre {
            //         Some(t) => {
            //             pre = t.next.as_ref();
            //             index -= 1;
            //         }

            //         _ => {
            //             return CommResult::Err;
            //         }
            //     }
            // }
            // let newNode = Rc::new(
            //     Node {data: elem, next: pre.unwrap().next.clone()}
            // );
            // // pre.unwrap().as_ref().borrow_mut().next = Some(newNode);
            // // todo!
            // let c  =pre.unwrap().next.as;
            // pre.unwrap().next = Some(newNode);
            CommResult::Ok
        }

        fn delete(&mut self, index: usize) -> Result<T, ()> {
            Err(())
        }

        fn traverse<F: Fn(&T)>(&self, func: F) {

        }

        fn merge(&mut self, other: Self) {

        }
    }

}