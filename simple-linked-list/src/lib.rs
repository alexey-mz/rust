use std::iter::FromIterator;
#[derive(Clone)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}
#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut result: usize = 0;
        let mut next = &self.head;
        while let Some(item) = next {
            result += 1;
            next = &item.next;
        }
        result
    }

    pub fn push(&mut self, _element: T) {
        let new_item = Box::new(Node{
            data: _element,
            next: self.head.take()
        });
        self.head = Some(new_item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|x| {
            self.head = x.next;
            x.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|x| &x.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        let mut head = self.head;
        while let Some(item) = head {
            reversed.push(item.data);
            head = item.next;
        }
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for i in _iter {
            list.push(i);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = vec![];
        let mut next = self.head;
        while let Some(item) = next {
            v.push(item.data);
            next = item.next;
        }
        v.reverse();
        v
    }
}
