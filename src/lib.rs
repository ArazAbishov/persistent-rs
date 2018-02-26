use std::mem;
use std::sync::Arc;

#[cfg(not(small_branch))]
const BRANCH_FACTOR: usize = 32;

#[cfg(small_branch)]
const BRANCH_FACTOR: usize = 4;

#[cfg(not(small_branch))]
const BITS_PER_LEVEL: usize = 5;

#[cfg(small_branch)]
const BITS_PER_LEVEL: usize = 2;

#[cfg(not(small_branch))]
macro_rules! no_children {
    () => {
        [None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,
         None, None, None, None,]
    }
}

#[cfg(small_branch)]
macro_rules! no_children {
    () => {
        [None, None, None, None]
    }
}

#[derive(Copy, Clone)]
struct Shift(usize);

#[derive(Copy, Clone)]
struct Index(usize);

impl Shift {
    fn inc(self) -> Shift {
        Shift(self.0 + BITS_PER_LEVEL)
    }
}

impl Index {
    fn child(self, shift: Shift) -> usize {
        (self.0 >> shift.0) & BRANCH_FACTOR - 1
    }

    fn element(self) -> usize {
        self.0 & BRANCH_FACTOR - 1
    }
}

enum Node<T> {
    Branch {
        children: [Option<Arc<Node<T>>>; BRANCH_FACTOR]
    },
    Leaf {
        elements: [Option<T>; BRANCH_FACTOR]
    },
}

// TODO: consider comparing performance of PVec where tail is backed by the Vec or plain array
struct PVec<T> {
    root: Option<Arc<Node<T>>>,
    root_size: Index,
    tail: [Option<T>; BRANCH_FACTOR],
    tail_size: Index,
    shift: Shift,
}

impl<T> PVec<T> {
    pub fn new() -> Self {
        PVec {
            root: Some(Arc::new(Node::Branch { children: no_children!() })),
            root_size: Index(0),
            tail: no_children!(),
            tail_size: Index(0),
            shift: Shift(0),
        }
    }

    pub fn push(&mut self, item: T) {
        self.tail[self.tail_size.0] = Some(item);
        self.tail_size.0 += 1;

        println!("Here one");

        if self.tail_size.0 == BRANCH_FACTOR {
            let tail = mem::replace(&mut self.tail, no_children!());

            self.root_size.0 += BRANCH_FACTOR;
            self.tail_size.0 = 0;

            self.push_tail(tail);
        }
    }

    fn push_tail(&mut self, tail: [Option<T>; BRANCH_FACTOR]) {
        if let Some(root) = self.root.as_mut() {
            let capacity = BRANCH_FACTOR << self.shift.0;

            if capacity == self.root_size.0 {
                println!("Need to grow this thingy.");

                let mut nodes = no_children!();
                nodes[0] = Some(root.clone());

                *root = Arc::new(Node::Branch { children: nodes });
            }

            // push the tail down to the leaf node
            // update the path which this tail has affected along the way
        } else {
            // no root, meaning that we didn't have any values at all
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        return self.tail[index].as_ref();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(small_branch))]
    fn shift_must_return_correct_index() {
        let index = Index(141);

        let shift_0 = Shift(0);
        let shift_1 = shift_0.inc();
        let shift_2 = shift_1.inc();
        let shift_3 = shift_2.inc();
        let shift_4 = shift_3.inc();
        let shift_5 = shift_4.inc();
        let shift_6 = shift_5.inc();
        let shift_7 = shift_6.inc();

        assert_eq!(index.element(), 0b01101);
        assert_eq!(index.child(shift_0), 0b01101);
        assert_eq!(index.child(shift_1), 0b00100);
        assert_eq!(index.child(shift_2), 0b00000);
        assert_eq!(index.child(shift_3), 0b00000);
        assert_eq!(index.child(shift_4), 0b00000);
        assert_eq!(index.child(shift_5), 0b00000);
        assert_eq!(index.child(shift_6), 0b00000);
        assert_eq!(index.child(shift_7), 0b00000);
    }

    #[test]
    fn new_must_return_correctly_initialized_pvec_instance() {
        let mut vec = PVec::new();

        for i in 0..33 {
            vec.push(i);
        }

        for i in 0..33 {
            assert_eq!(*vec.get(i).unwrap(), i);
        }
    }
}