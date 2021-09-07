
pub mod stack {
    use crate::{linearTable::{LinearTable, list::{DLinkList, List}, vector::Vector}};
    type Container<T> = DLinkList<T>;
    pub struct Stack<T>
        where T: PartialEq + PartialOrd + Clone
    {
        
        container: Container<T>,
        len: usize,
    }

    impl<T> Stack<T>
        where T: PartialEq + PartialOrd + Clone
    {
        pub fn new() -> Self {
            Stack { container: Container::<T>::new(), len: 0, }
        }

        pub fn destroy(&mut self) {
            self.container.destroy();
            self.len = 0;
        }
        
        pub fn clear(&mut self) {
            self.container.clear();
            self.len = 0;
        }

        pub fn empty(&self) -> bool {
            self.len == 0
        }

        pub fn get_top(&self) -> Option<T> {
            if self.empty() {
                None
            } else {
                Some(self.container.get_elem(self.len - 1).unwrap())
            }
        }

        pub fn push(&mut self, elem: T) {
            self.container.insert(elem, self.len);
            self.len += 1;
        }

        pub fn pop(&mut self) -> Option<T>{
            if self.empty() {
                None
            } else {
                self.len -= 1;
                Some(self.container.delete(self.len).unwrap())
            }
        }

        pub fn traverse(&self, func: fn(&T)) {
            self.container.traverse(func);
        }
    }
}

pub mod queue {
    use crate::{linearTable::{LinearTable, list::{DLinkList, List}, vector::Vector}};
    type Container<T> = DLinkList<T>;
    pub struct Queue<T> 
        where T: PartialEq + PartialOrd + Clone
    {
        container: Container<T>,
        len: usize,
    }

    impl <T> Queue<T>
     where T: PartialEq + PartialOrd + Clone
    {
        pub fn new() -> Self {
            Queue {container: Container::new(), len: 0, }
        }

        pub fn destroy(&mut self) {
            self.container.destroy();
            self.len = 0;
        }

        pub fn clear(&mut self) {
            self.container.clear();
            self.len = 0;
        }

        pub fn empty(&self) -> bool {
            self.len == 0
        } 

        pub fn get_head(&self) -> Option<T> {
            self.container.get_front()
        }

        pub fn push(&mut self, elem: T) {
            self.container.push_back(elem);
            self.len += 1;
        }

        pub fn pop(&mut self) -> Result<T, ()>{
            self.len -= 1;
            self.container.pop_front()
        }
    }
}