pub trait LinearTable<T>
where
    T: PartialEq + PartialOrd + Clone,
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

    fn push_back(&mut self, elem: T);
    fn push_front(&mut self, elem: T);
    fn pop_back(&mut self) -> Result<T, ()>;
    fn pop_front(&mut self) -> Result<T, ()>;
    fn get_front(&self) -> Option<T>;
    fn get_back(&self) -> Option<T>;


    fn traverse(&self, func: fn(&T));

    // fn merge(&mut self, other: Self) where Self:Sized;
}

pub enum CommResult {
    Ok,
    Err,
}

pub mod vector {

    use super::{CommResult, LinearTable};

    pub struct Vector<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        vector: Vec<T>,
    }

    impl<T> Vector<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        pub fn new() -> Self {
            Vector {
                vector: Vec::<T>::new(),
            }
        }
    }
    impl<T> LinearTable<T> for Vector<T>
    where
        T: PartialEq + PartialOrd + Clone,
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
                    return Some(i);
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

        fn push_back(&mut self, elem: T) {
            self.vector.push(elem);
        }

        fn push_front(&mut self, elem: T) {
            self.insert(elem, 0);
        }

        fn pop_back(&mut self) -> Result<T, ()> {
            match self.vector.pop() {
                Some(t) => Ok(t),
                None => Err(()),
            }
        }

        fn pop_front(&mut self) -> Result<T, ()> {
            self.delete(0)
        }

        fn get_back(&self) -> Option<T> {
            self.get_elem(self.length() - 1)
        }

        fn get_front(&self) -> Option<T> {
            self.get_elem(0)
        }

        fn traverse(&self, func: fn(&T)) {
            for v in self.vector.iter() {
                func(v);
            }
        }
    }

    impl<T> Vector<T>
    where
        T: PartialOrd + PartialEq + Clone,
    {
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
                    _ => break,
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

    use std::{
        borrow::{Borrow, BorrowMut},
        cell::RefCell,
        convert::TryInto,
        rc::{Rc, Weak},
    };

    use super::{CommResult, LinearTable};

    pub struct List<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        head: Rc<RefCell<Node<T>>>,
        len: usize,
    }

    struct Node<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        data: Option<T>,
        next: Option<Rc<RefCell<Node<T>>>>,
    }

    impl<T> List<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        pub fn new() -> Self {
            List {
                head: Rc::new(RefCell::new(Node {
                    data: None,
                    next: None,
                })),
                len: 0,
            }
        }
    }

    impl<T> LinearTable<T> for List<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        fn destroy(&mut self) {
            self.head.as_ref().borrow_mut().next = None;
            self.len = 0;
        }

        fn clear(&mut self) {
            self.head.as_ref().borrow_mut().next = None;
            self.len = 0;
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
            let mut pre = self.head.as_ref().borrow().next.clone();
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
            if self.empty() {
                return None;
            }
            let mut pre = self.head.as_ref().borrow().next.clone().unwrap();
            let mut cur = pre.as_ref().borrow().next.clone();
            while let Some(t) = cur.clone() {
                if t.as_ref().borrow().data.clone().unwrap() == elem {
                    return pre.as_ref().borrow().data.clone();
                }
                pre = cur.unwrap();
                cur = pre.as_ref().borrow().next.clone();
            }
            None
        }

        fn next_elem(&self, elem: T) -> Option<T> {
            if self.empty() {
                return None;
            }
            let mut cur = self.head.as_ref().borrow().next.clone().unwrap();
            let mut next = cur.as_ref().borrow().next.clone();
            while let Some(t) = next.clone() {
                if cur.as_ref().borrow().data.clone().unwrap() == elem {
                    return t.as_ref().borrow().data.clone();
                }
                cur = next.unwrap();
                next = cur.as_ref().borrow().next.clone();
            }
            None
        }

        fn insert(&mut self, elem: T, index: usize) -> CommResult {
            if index > self.len {
                return CommResult::Err;
            }
            let mut index = index;
            if index == 0 {
                let newNode = Rc::new(RefCell::new(Node {
                    data: Some(elem),
                    next: self.head.as_ref().borrow().next.clone(),
                }));
                self.head.as_ref().borrow_mut().next = Some(newNode);
                self.len += 1;
                return CommResult::Ok;
            }
            let mut pre = self.head.as_ref().borrow().next.clone();

            while index > 1 {
                match pre {
                    Some(t) => {
                        pre = t.as_ref().borrow().next.clone();
                        index -= 1;
                    }
                    _ => {
                        return CommResult::Err;
                    }
                }
            }

            let newNode = Rc::new(RefCell::new(Node {
                data: Some(elem),
                next: pre.clone().unwrap().as_ref().borrow().next.clone(),
            }));
            pre.unwrap().as_ref().borrow_mut().next = Some(newNode);
            self.len += 1;
            CommResult::Ok
        }

        fn delete(&mut self, index: usize) -> Result<T, ()> {
            if index >= self.len {
                return Err(());
            }
            let mut index = index;
            let mut pre = self.head.clone();
            while index > 0 {
                index -= 1;
                pre = pre.clone().as_ref().borrow().next.clone().unwrap();
            }
            let tmp = pre.as_ref().borrow().next.clone().unwrap();
            let ans = tmp.as_ref().borrow().data.clone().unwrap();
            pre.as_ref().borrow_mut().next = tmp.as_ref().borrow().next.clone();
            self.len -= 1;
            Ok(ans)
        }

        fn push_back(&mut self, elem: T) {
            self.insert(elem, self.len);
        }

        fn push_front(&mut self, elem: T) {
            self.insert(elem, 0);
        }

        fn pop_front(&mut self) -> Result<T, ()> {
            self.delete(0)
        }

        fn pop_back(&mut self) -> Result<T, ()> {
            self.delete(self.len - 1)
        }

        fn get_front(&self) -> Option<T> {
            self.get_elem(0)
        }

        fn get_back(&self) -> Option<T> {
            self.get_elem(self.length() - 1)
        }

        fn traverse(&self, func: fn(&T)) {
            let mut cur = self.head.as_ref().borrow().next.clone();
            while let Some(t) = cur {
                func(t.as_ref().borrow().data.as_ref().unwrap());
                cur = t.as_ref().borrow().next.clone();
            }
        }
    }
    impl<T> List<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        fn merge(&mut self, other: Self) {
            let mut left = self.head.as_ref().borrow().next.clone();
            let mut right = other.head.as_ref().borrow().next.clone();
            let newHead = Rc::new(RefCell::new(Node {
                data: self.head.as_ref().borrow().data.clone(),
                next: None,
            }));
            let mut tmp = newHead.clone();
            loop {
                match (left.clone(), right.clone()) {
                    (Some(a), Some(b)) => {
                        if a.as_ref().borrow().data.clone().unwrap()
                            > b.as_ref().borrow().data.clone().unwrap()
                        {
                            tmp.as_ref().borrow_mut().next = Some(b.clone());

                            right = b.as_ref().borrow().next.clone();
                        } else {
                            tmp.as_ref().borrow_mut().next = Some(a.clone());
                            left = a.as_ref().borrow().next.clone();
                        }
                    }
                    _ => {
                        break;
                    }
                }
                tmp = tmp.clone().as_ref().borrow().next.clone().unwrap();
            }
            while let Some(t) = left {
                tmp.as_ref().borrow_mut().next = Some(t.clone());
                left = t.as_ref().borrow().next.clone();
                tmp = t.clone();
            }
            while let Some(t) = right {
                tmp.as_ref().borrow_mut().next = Some(t.clone());
                right = t.as_ref().borrow().next.clone();
                tmp = t.clone();
            }
            self.head = newHead;
            self.len += other.len;
        }
    }

    pub struct DLinkList<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        head: Rc<RefCell<DNode<T>>>,
        tail: Rc<RefCell<DNode<T>>>,
        len: usize,
    }

    struct DNode<T>
    where
        T: PartialOrd + PartialEq + Clone,
    {
        pre: Option<Weak<RefCell<DNode<T>>>>,
        next: Option<Rc<RefCell<DNode<T>>>>,
        data: Option<T>,
    }

    impl<T> LinearTable<T> for DLinkList<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        fn destroy(&mut self) {
            self.head.as_ref().borrow_mut().next = Some(self.tail.clone());
            self.tail.as_ref().borrow_mut().pre = Some(Rc::downgrade(&self.head));
            self.len = 0;
        }

        fn clear(&mut self) {
            self.head.as_ref().borrow_mut().next = Some(self.tail.clone());
            self.tail.as_ref().borrow_mut().pre = Some(Rc::downgrade(&self.head));
            self.len = 0;
        }

        fn empty(&self) -> bool {
            self.len == 0
        }

        fn length(&self) -> usize {
            self.len
        }

        fn get_elem(&self, index: usize) -> Option<T> {
            if index >= self.len {
                return None;
            }
            let mut index = index;
            let mut cur = self.head.as_ref().borrow().next.clone();
            while index > 0 {
                match cur {
                    Some(t) => {
                        cur = t.as_ref().borrow().next.clone();
                        index -= 1;
                    }
                    _ => return None,
                }
            }
            cur.unwrap().as_ref().borrow().data.clone()
        }

        fn next_elem(&self, elem: T) -> Option<T> {
            None
        }

        fn prior_elem(&self, elem: T) -> Option<T> {
            None
        }

        fn locate_elem(&self, elem: T) -> Option<usize> {
            let mut index = self.len;
            let mut cur = self.head.as_ref().borrow().next.clone();
            while index > 0 {
                match cur {
                    Some(t) => {
                        if t.as_ref().borrow().data.clone().unwrap() == elem {
                            return Some(self.len - index);
                        }
                        index -= 1;
                        cur = t.as_ref().borrow().next.clone();
                    }
                    _ => {
                        break;
                    }
                }
            }
            None
        }

        fn insert(&mut self, elem: T, index: usize) -> CommResult {
            if index > self.len {
                return CommResult::Err;
            }
            let mut index = index;
            let mut pre = self.head.clone();
            while index > 0 {
                pre = pre.clone().as_ref().borrow().next.clone().unwrap();
                index -= 1;
            }

            let newNode = Rc::new(RefCell::new(DNode {
                data: Some(elem),
                next: pre.clone().as_ref().borrow().next.clone(),
                pre: Some(Rc::downgrade(&pre.clone())),
            }));

            newNode
                .as_ref()
                .borrow()
                .next
                .clone()
                .unwrap()
                .as_ref()
                .borrow_mut()
                .pre = Some(Rc::downgrade(&newNode));

            pre.as_ref().borrow_mut().next = Some(newNode);
            self.len += 1;
            CommResult::Ok
        }

        fn delete(&mut self, index: usize) -> Result<T, ()> {
            if index >= self.len {
                return Err(());
            }
            let mut index = index;
            let mut pre = self.head.clone();
            while index > 0 {
                index -= 1;
                pre = pre.clone().as_ref().borrow().next.clone().unwrap();
            }
            let tmp = pre.as_ref().borrow().next.clone().unwrap();
            let ans = tmp.as_ref().borrow().data.clone().unwrap();
            pre.as_ref().borrow_mut().next = tmp.as_ref().borrow().next.clone();
            tmp.as_ref()
                .borrow()
                .next
                .clone()
                .unwrap()
                .as_ref()
                .borrow_mut()
                .pre = Some(Rc::downgrade(&pre));
            self.len -= 1;
            Ok(ans)
        }

        fn push_back(&mut self, elem: T) {
            let newNode = Rc::new(RefCell::new(DNode {
                next: Some(self.tail.clone()),
                pre: self.tail.as_ref().borrow().pre.clone(),
                data: Some(elem),
            }));

            match self.tail.as_ref().borrow().pre.clone().unwrap().upgrade() {
                Some(t) => {
                    t.as_ref().borrow_mut().next = Some(newNode.clone());
                }
                _ => {
                    return;
                }
            }
            self.tail.as_ref().borrow_mut().pre = Some(Rc::downgrade(&newNode));
            self.len += 1;
        }

        fn push_front(&mut self, elem: T) {
            self.insert(elem, 0);
        }

        fn pop_front(&mut self) -> Result<T, ()> {
            self.delete(0)
        }

        fn pop_back(&mut self) -> Result<T, ()> {
            if self.empty() {
                Err(())
            } else {
                let target = self.tail.as_ref().borrow().pre.clone().unwrap().upgrade();
                match target {
                    Some(t) => {
                        let pre = t.as_ref().borrow().pre.clone().unwrap().upgrade();
                        match pre {
                            Some(p) => {
                                p.as_ref().borrow_mut().next = Some(self.tail.clone());
                                self.tail.as_ref().borrow_mut().pre = Some(Rc::downgrade(&p));
                                self.len -= 1;
                                return Ok(t.as_ref().borrow().data.clone().unwrap())
                            }
                            _ => {
                                return Err(());
                            }
                        }
                    }
                    _ => {
                        return Err(());
                    }
                }
            }
        }

        fn get_front(&self) -> Option<T> {
            self.get_elem(0)
        }

        fn get_back(&self) -> Option<T> {
            if self.empty() {
                None
            } else {
                match self.tail.as_ref().borrow().pre.clone().unwrap().upgrade() {
                    Some(t) => {
                        t.as_ref().borrow().data.clone()
                    }
                    _ => {
                        None
                    }
                }
            }
        }

        fn traverse(&self, func: fn(&T)) {
            let mut cur = self.head.as_ref().borrow().next.clone();
            let mut index = self.len;
            while index > 0 {
                func(
                    cur.clone()
                        .unwrap()
                        .as_ref()
                        .borrow()
                        .data
                        .as_ref()
                        .unwrap(),
                );
                cur = cur.unwrap().as_ref().borrow().next.clone();
                index -= 1;
            }
        }
    }

    impl<T> DLinkList<T>
    where
        T: PartialEq + PartialOrd + Clone,
    {
        pub fn new() -> Self {
            let new_list = DLinkList {
                head: Rc::new(RefCell::new(DNode {
                    pre: None,
                    next: None,
                    data: None,
                })),
                tail: Rc::new(RefCell::new(DNode {
                    pre: None,
                    next: None,
                    data: None,
                })),
                len: 0,
            };

            new_list.head.as_ref().borrow_mut().next = Some(new_list.tail.clone());
            new_list.tail.as_ref().borrow_mut().pre = Some(Rc::downgrade(&new_list.head));
            new_list
        }
    }
}
