#[warn(dead_code)]

use std::borrow::BorrowMut;
use std::mem;

pub const INVALID_INDEX: usize = <usize>::max_value();
pub const RESIZE_AMOUNT: usize = 5;
pub const EMPTY: usize = 0;

type position = usize;

pub struct List <T> {
    data: Vec<T>
}

impl <T> List <T> {

    pub fn new () -> Self {
        Self {
            data: Vec::with_capacity(RESIZE_AMOUNT),            
        }
    }

    pub fn print_list(&self) where T: std::fmt::Debug {

        if self.data.is_empty() {
            println!("The list is empty!");
            return;
        }

        for element in self.data.iter() {
            println!("{:?}", element);
        }
    }

    pub fn last(&self) -> Option<&T> {
       if self.data.is_empty() {
            return None;
        }        
        self.data.last()
    }

    pub fn first(&self) -> Option<&T> {
        if self.data.is_empty() {
            return None;
        }
        self.data.first()
    }

    pub fn add(&mut self, element: T) {
        self.data.push(element);
    }

    pub fn insert(&mut self, element: T, at: position) where T: std::cmp::Ord {
        if at >= self.data.len() {
            return;
        }

        self.data.insert(at, element);
    }

    pub fn remove_element(&mut self, element: T) where T: std::cmp::Ord {
        if self.data.is_empty() {
            return;
        }

        let index = self.get_index(element);
        if index == INVALID_INDEX {
            return;
        }

        self.data.remove(index as usize);
    }

    pub fn get_index(&mut self, element: T) -> usize where T: std::cmp::Ord {

        if self.data.is_empty() {
            return 0;
        }

        match self.data.binary_search_by(|probe| probe.cmp(&element)) {
            Ok(pos) => { return pos } 
            Err(pos) => { return INVALID_INDEX },
        }
    }

}
