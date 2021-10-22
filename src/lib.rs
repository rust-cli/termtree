use std::collections::VecDeque;
use std::fmt::{self, Display};
use std::rc::Rc;

/// a simple recursive type which is able to render its
/// components in a tree-like format
#[derive(Debug)]
pub struct Tree<D: Display> {
    root: D,
    leaves: Vec<Tree<D>>,
    multiline: bool,
}

impl<D: Display> Tree<D> {
    pub fn new(root: D, leaves: Vec<Tree<D>>) -> Tree<D> {
        Tree {
            root,
            leaves,
            multiline: false,
        }
    }

    pub fn root(root: D) -> Tree<D> {
        Tree {
            root,
            leaves: Vec::new(),
            multiline: false,
        }
    }

    /// Ensure all lines for `root` are indented
    pub fn with_multiline(mut self, yes: bool) -> Self {
        self.multiline = yes;
        self
    }

    /// Ensure all lines for `root` are indented
    pub fn set_multiline(&mut self, yes: bool) -> &mut Self {
        self.multiline = yes;
        self
    }

    pub fn push(&mut self, leaf: Tree<D>) -> &mut Self {
        self.leaves.push(leaf);
        self
    }
}

impl<D: Display> Extend<D> for Tree<D> {
    fn extend<T: IntoIterator<Item = D>>(&mut self, iter: T) {
        self.leaves.extend(iter.into_iter().map(Tree::root))
    }
}

impl<D: Display> Extend<Tree<D>> for Tree<D> {
    fn extend<T: IntoIterator<Item = Tree<D>>>(&mut self, iter: T) {
        self.leaves.extend(iter)
    }
}

impl<D: Display> Display for Tree<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.root)?;
        let mut queue = DisplauQueue::new();
        let no_space = Rc::new(Vec::new());
        enqueue_leaves(&mut queue, self, no_space);
        while let Some((last, leaf, spaces)) = queue.pop_front() {
            let mut prefix = if last { "└──" } else { "├──" };

            if leaf.multiline {
                let rest_prefix = if last { "   " } else { "|  " };
                debug_assert_eq!(prefix.chars().count(), rest_prefix.chars().count());

                let root = leaf.root.to_string();
                for line in root.lines() {
                    // print single line
                    for s in spaces.as_slice() {
                        if *s {
                            write!(f, "    ")?;
                        } else {
                            write!(f, "|   ")?;
                        }
                    }
                    writeln!(f, "{} {}", prefix, line)?;
                    prefix = rest_prefix;
                }
            } else {
                // print single line
                for s in spaces.as_slice() {
                    if *s {
                        write!(f, "    ")?;
                    } else {
                        write!(f, "|   ")?;
                    }
                }
                writeln!(f, "{} {}", prefix, leaf.root)?;
            }

            // recurse
            if !leaf.leaves.is_empty() {
                let s: &Vec<bool> = &spaces;
                let mut child_spaces = s.clone();
                child_spaces.push(last);
                let child_spaces = Rc::new(child_spaces);
                enqueue_leaves(&mut queue, leaf, child_spaces);
            }
        }
        Ok(())
    }
}

type DisplauQueue<'t, D> = VecDeque<(bool, &'t Tree<D>, Rc<Vec<bool>>)>;

fn enqueue_leaves<'t, D: Display>(
    queue: &mut DisplauQueue<'t, D>,
    parent: &'t Tree<D>,
    spaces: Rc<Vec<bool>>,
) {
    for (i, leaf) in parent.leaves.iter().rev().enumerate() {
        let last = i == 0;
        queue.push_front((last, leaf, spaces.clone()));
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;
    #[test]
    fn render_tree_root() {
        let tree = Tree::root("foo");
        assert_eq!(format!("{}", tree), "foo\n")
    }

    #[test]
    fn render_tree_with_leaves() {
        let tree = Tree::new("foo", vec![Tree::new("bar", vec![Tree::root("baz")])]);
        assert_eq!(
            format!("{}", tree),
            r#"foo
└── bar
    └── baz
"#
        )
    }

    #[test]
    fn render_tree_with_multiple_leaves() {
        let tree = Tree::new("foo", vec![Tree::root("bar"), Tree::root("baz")]);
        assert_eq!(
            format!("{}", tree),
            r#"foo
├── bar
└── baz
"#
        )
    }

    #[test]
    fn render_tree_with_multiline_leaf() {
        let tree = Tree::new(
            "foo",
            vec![
                Tree::root("hello\nworld").with_multiline(true),
                Tree::root("goodbye\nworld").with_multiline(true),
            ],
        );
        assert_eq!(
            format!("{}", tree),
            r#"foo
├── hello
|   world
└── goodbye
    world
"#
        )
    }
}
