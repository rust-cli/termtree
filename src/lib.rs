use std::fmt::{self, Display};

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

    fn display_leaves(
        f: &mut fmt::Formatter,
        leaves: &[Tree<D>],
        spaces: Vec<bool>,
    ) -> fmt::Result {
        for (i, leaf) in leaves.iter().enumerate() {
            let last = i >= leaves.len() - 1;
            let mut prefix = if last { "└──" } else { "├──" };

            if leaf.multiline {
                let rest_prefix = if last { "   " } else { "|  " };
                debug_assert_eq!(prefix.chars().count(), rest_prefix.chars().count());

                let root = leaf.root.to_string();
                for line in root.lines() {
                    // print single line
                    for s in &spaces {
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
                for s in &spaces {
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
                let mut clone = spaces.clone();
                clone.push(last);
                Self::display_leaves(f, &leaf.leaves, clone)?;
            }
        }
        write!(f, "")
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
        Self::display_leaves(f, &self.leaves, Vec::new())
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
