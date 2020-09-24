#[derive(Debug)]
struct List<T> {
    head: Option<Box<Node<T>>>,
}
impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None, }
    }

    pub fn push(&mut self, item: T) {
        if let Some(node) = &mut self.head {
            node.push(item)
        } else {
            self.head = Some(Box::new(Node {
                elem: item,
                next: None,
            }));
        }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        ListIter {
            list: self
        }
    }
}
struct ListIter<T> {
    list: List<T>
}

impl<T> Iterator for ListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        //SAFETY: we immediately replace self.list.head with the next node.
        let tmp = std::mem::replace(&mut self.list.head, unsafe{std::mem::zeroed()});
        if let Some(node) = tmp {
            let Node{next:next_node, elem: out} = *node;
            self.list.head = next_node;
            Some(out)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Node<T> {
    fn push(&mut self, item: T) {
        if let Some(node) = &mut self.next {
            node.push(item)
        } else {
            self.next = Some(Box::new(Self {
                elem: item,
                next: None,
            }));
        }
    }
}

macro_rules! list {
    ($($e:expr), *) => {
        {
            let mut m = List::new();
            $(
                m.push($e);
            )*
            m
        }
    };
}

fn main() {
    let list = list![5, 3, 8];
    for i in list {
        println!("{}",i);
    }
}