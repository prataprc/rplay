#! /usr/bin/env rustr

#![feature(generic_associated_types)]

// Associated type constructors.

fn main() {
    let mut list = List::new();
    list.prepend(10);
    list.prepend(20);
    list.prepend(30);

    let mut iter = list.iter();
    while let Some(value) = iter.next() {
        println!("{}", value);
    }
}

/// Very simple linked list. If `cell` is `None`,
/// the list is empty.
pub struct List<T> {
    cell: Option<Box<ListCell<T>>>,
}

/// A single cell in a non-empty list. Stores one
/// value and then another list.
struct ListCell<T> {
    value: T,
    next: List<T>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { cell: None }
    }

    pub fn prepend(&mut self, value: T) {
        // get ahold of the current head of the list, if any
        let old_head = List{cell: self.cell.take()};

        // Create a new cell to serve as the new head of the list,
        // and then store it in `self.cell`.
        let cell = ListCell { value: value, next: old_head };
        self.cell = Some(Box::new(cell));
    }

    pub fn iter<'iter>(&'iter self) -> ListIter<'iter, T> {
        ListIter { cursor: self }
    }
}

/// Iterator over linked lists.
pub struct ListIter<'iter, T> where T: 'iter {
    cursor: &'iter List<T>
}

impl<'iter, T> Iterator for ListIter<'iter, T> {
    type Item = &'iter T;
    fn next(&mut self) -> Option<&'iter T> {
        // If the list is non-empty, borrow a reference
        // to the cell (`cell`).
        if let Some(ref cell) = self.cursor.cell {
            // Point the cursor at the next cell.
            self.cursor = &cell.next;

            // Return reference to the value in the
            // the current cell.
            Some(&cell.value)
        } else {
            // List is empty, return `None`.
            None
        }
    }
}

trait Collection<T> {
    // create an empty collection of this type:
    fn empty() -> Self;

    // add `value` to this collection in some way:
    fn add(&mut self, value: T);

    // iterate over this collection:
    fn iterate(&self) -> Self::Iter;

    // the type of an iterator for this collection (e.g., `ListIter`)
    type Iter<'iter>: Iterator<Item=T>;
}

impl<T> Collection<T> for List<T> {
    fn empty() -> List<T> {
        List::new()
    }

    fn add(&mut self, value: T) {
        self.prepend(value);
    }

    fn iterate<'iter>(&'iter self) -> ListIter<'iter, T> {
        self.iter()
    }

    type Iter<'iter> = ListIter<'iter, T>;
}
