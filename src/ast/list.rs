use std::rc::Rc;
#[derive(Debug, Clone)]
pub struct List<T> {
    head: Link<T>
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug, Clone)]
struct Node<T> {
    elm: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn wrap(elem: T) -> Self {
        let l = List { head: None };
        l.append(elem)
    }

    pub fn append(&self, elem: T) -> List<T> {
        List { head: Some(Rc::new(Node {
            elm: elem,
            next: self.head.clone(),
        }))}
    }

    pub fn copy(&self) -> List<T> {
        List { head: self.head.clone() }
    }

    pub fn len(&self) -> usize {
        if self.head.is_some() {
            1 + self.tail().len()
        } else {
            0
        }
    }

    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elm)
    }

    pub fn tail_tip(&self) -> Option<&T> {
        let mut i = self.iter();
        let mut val = i.next();
        while let Some(v) = i.next() {
            val = Some(v);
        }
        val
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }
}

impl<T> Drop for List<T> {
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

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elm
        })
    }
}
