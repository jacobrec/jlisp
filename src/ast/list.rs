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

    pub fn cons(elem: T, rest: &Self) -> Self {
        List { head: Some(Rc::new(Node {
            elm: elem,
            next: rest.head.clone(),
        }))}
    }

    pub fn append(&self, elem: T) -> Self {
        List::cons(elem, self)
    }

    pub fn copy(&self) -> Self {
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let l = List::new();
        let a = List::cons(3, &l);
        let b = List::cons(2, &a);
        let c = List::cons(1, &b);

        assert_eq!(Some(&3), a.head());
        assert_eq!(Some(&2), b.head());
        assert_eq!(Some(&1), c.head());
    }

    #[test]
    fn test_2() {
        let l = List::new()
            .append(3)
            .append(2)
            .append(1);

        assert_eq!(Some(&3), l.tail().tail().head());
        assert_eq!(Some(&2), l.tail().head());
        assert_eq!(Some(&1), l.head());
    }
}
