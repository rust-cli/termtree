use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Tree<D: Display>(D, Vec<Tree<D>>);

impl<D: Display> Tree<D> {
    pub fn new(root: D, leaves: Vec<Tree<D>>) -> Tree<D> {
        Tree(root, leaves)
    }

    pub fn root(root: D) -> Tree<D> {
        Tree(root, Vec::new())
    }

    pub fn push(&mut self, leaf: Tree<D>) -> &mut Self {
        self.1.push(leaf);
        self
    }

    fn rec(f: &mut fmt::Formatter, leaves: &Vec<Tree<D>>, spaces: Vec<bool>) -> fmt::Result {
        for (i, leaf) in leaves.iter().enumerate() {
            let last = i >= leaves.len() - 1;
            let mut clone = spaces.clone();
            // print single line
            for s in &spaces {
                if *s {
                    let _ = write!(f, "    ");
                } else {
                    let _ = write!(f, "|   ");
                }
            }
            if last {
                let _ = writeln!(f, "└── {}", leaf.0);
            } else {
                let _ = writeln!(f, "├── {}", leaf.0);
            }

            // recurse
            if !leaf.1.is_empty() {
                clone.push(last);
                let _ = Self::rec(f, &leaf.1, clone);
            }
        }
        write!(f, "")
    }
}

impl<D: Display> Display for Tree<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let _ = writeln!(f, "{}", self.0);
        Self::rec(f, &self.1, Vec::new())
    }
}
