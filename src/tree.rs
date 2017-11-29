
use std::collections::VecDeque;

/// A rose tree structure
#[derive(Clone, PartialEq, Eq)]
pub struct Tree<T> {
    root: Option<Node<T>>
}

#[derive(Clone, PartialEq, Eq)]
struct Node<T> {
    pub data: T,
    pub sub_forest: Vec<Node<T>>
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Tree::new()
    }
}

impl<T> Tree<T> {
    /// Creates a new tree with no branches
    pub fn new() -> Self {
        Tree {
            root: None
        }
    }

    /// Creates a new tree with the supplied data
    pub fn with_data(data: T) -> Self {
        Tree {
            root: Some(Node::new(data))
        }
    }

    /// Either replaces an empty root with `other`, or appends `other` to the list of
    /// subtrees of `self`.
    ///
    /// If `other.root` is none, this is a no-op.
    pub fn insert(&mut self, other: Self) {
        if other.root.is_none() {
            return;
        }

        match self.root {
            None => self.root = other.root,
            // Unwrap is safe because we do nothing if the other root is None
            Some(ref mut node) => node.sub_forest.push(other.root.unwrap()),
        }
    }

    /// Consumes the tree returning the elements in a preorder
    pub fn flatten(self) -> Vec<T> {
        self.dfs().into_iter().collect()
    }

    /// Consumes the tree returning a newtype for breadth first searching
    pub fn bfs(self) -> BreadthFirstSearch<T> {
        self.into()
    }
    
    /// Consumes the tree returning a newtype for depth first searching
    pub fn dfs(self) -> DepthFirstSearch<T> {
        self.into()
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            sub_forest: Vec::new()
        }
    }
}

/// A wrapper around a tree for performing breadth first searches
#[derive(Clone, PartialEq, Eq)]
pub struct BreadthFirstSearch<T>(Tree<T>);
/// A wrapper around a tree for performing depth first searches
#[derive(Clone, PartialEq, Eq)]
pub struct DepthFirstSearch<T>(Tree<T>);

impl<T> IntoIterator for BreadthFirstSearch<T> {
    type Item = T;
    type IntoIter = BfsIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.0.root {
            None => BfsIntoIter {
                next: None,
                cur: vec![].into_iter(),
                rest: VecDeque::new(),
            },
            Some(Node { data, sub_forest }) => BfsIntoIter {
                next: Some(data),
                cur: sub_forest.into_iter(),
                rest: VecDeque::new(),
            },
        }
    }
}

pub struct BfsIntoIter<T> {
    next: Option<T>,
    cur: ::std::vec::IntoIter<Node<T>>,
    rest: VecDeque<Vec<Node<T>>>
}

impl<T> Iterator for BfsIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ret = self.next.take();
        if let Some(Node { data, sub_forest }) = match self.cur.next() {
            None => if let Some(v) = self.rest.pop_front() {
                assert!(v.len() > 0, "length should always be greater than 0");
                self.cur = v.into_iter();
                self.cur.next()
            } else {
                None
            },
            something => something
        } {
            self.next = Some(data);
            if sub_forest.len() > 0 {
                self.rest.push_back(sub_forest);
            }
        }
        ret
    }
}

impl<T> IntoIterator for DepthFirstSearch<T> {
    type Item = T;
    type IntoIter = DfsIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        match self.0.root {
            None => DfsIntoIter {
                next: None,
                rest: VecDeque::new(),
            },
            Some(Node { data, sub_forest }) => DfsIntoIter {
                next: Some(data),
                rest: sub_forest.into(),
            },
        }
    }
}

pub struct DfsIntoIter<T> {
    next: Option<T>,
    rest: VecDeque<Node<T>>,
}

impl<T> Iterator for DfsIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ret = self.next.take();
        if let Some(Node { data, sub_forest }) = self.rest.pop_front() {
            self.next = Some(data);
            for node in sub_forest.into_iter().rev() {
                self.rest.push_front(node);
            }
        }
        ret
    }
}

macro_rules! search_impls {
    ($typename:ident) => {
        impl<T> ::std::ops::Deref for $typename<T> {
            type Target = Tree<T>;
            fn deref(&self) -> &Tree<T> {
                &self.0
            }
        }

        impl<T> From<Tree<T>> for $typename<T> {
            fn from(tree: Tree<T>) -> Self {
                $typename(tree)
            }
        }
    }
}

search_impls!(BreadthFirstSearch);
search_impls!(DepthFirstSearch);

#[cfg(test)]
mod test {
    use super::*;
    fn test_tree() -> Tree<i64> {
        let mut x = Tree::with_data(1);
        let (mut y, z) = (Tree::with_data(2), Tree::with_data(3));
        let (y1, y2) = (Tree::with_data(4), Tree::with_data(5));
        y.insert(y1); y.insert(y2);
        x.insert(y); x.insert(z);
        x
    }

    #[test]
    fn bfs_iter() {
        let v = test_tree().bfs().into_iter().collect::<Vec<_>>();
        assert_eq!(vec![1,2,3,4,5], v);
    }

    #[test]
    fn dfs_iter() {
        let v = test_tree().dfs().into_iter().collect::<Vec<_>>();
        assert_eq!(vec![1,2,4,5,3], v);
    }

    #[test]
    fn flatten() {
        let v = test_tree().flatten();
        assert_eq!(vec![1,2,4,5,3], v);
    }
}
