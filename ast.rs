use std::rc::Rc;
#[derive(Debug)]
pub struct List {
    head: Link
}

type Link = Option<Rc<Node>>;

#[derive(Debug)]
struct Node {
    elm: AtomOrList,
    next: Link
}

#[derive(Debug)]
pub enum AtomOrList {
    Atom(Atom),
    List(List),
}

#[derive(Debug)]
pub enum Atom {
    AIdentifier(String),
    AString(String),
    // AFloat(f64),
    AInteger(isize),
    ATrue,
    AFalse,
    // AVector(),
    // AChar(char),
}


impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn wrap(elem: AtomOrList) -> Self {
        let l = List { head: None };
        l.append(elem)
    }

    pub fn append(&self, elem: AtomOrList) -> List {
        List { head: Some(Rc::new(Node {
            elm: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn copy(&self) -> List {
        List { head: self.head.clone() }
    }

    pub fn len(&self) -> usize {
        if self.head.is_some() {
            1 + self.tail().len()
        } else {
            0
        }
    }

    pub fn tail(&self) -> List {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&AtomOrList> {
        self.head.as_ref().map(|node| &node.elm)
    }

    pub fn tail_tip(&self) -> Option<&AtomOrList> {
        let mut i = self.iter();
        let mut val = i.next();
        while let Some(v) = i.next() {
            val = Some(v);
        }
        val
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a> {
    next: Option<&'a Node>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a AtomOrList;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elm
        })
    }
}
