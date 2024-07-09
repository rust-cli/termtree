use snapbox::assert_data_eq;
use snapbox::str;

use super::*;

#[test]
fn render_tree_root() {
    let tree = Tree::new("foo");
    assert_data_eq!(
        format!("{}", tree),
        str![[r#"
foo

"#]]
    );
}

#[test]
fn render_tree_with_leaves() {
    let tree = Tree::new("foo").with_leaves([Tree::new("bar").with_leaves(["baz"])]);
    assert_data_eq!(
        format!("{}", tree),
        str![[r#"
foo
└── bar
    └── baz

"#]]
    );
}

#[test]
fn render_tree_with_multiple_leaves() {
    let tree = Tree::new("foo").with_leaves(["bar", "baz"]);
    assert_data_eq!(
        format!("{}", tree),
        str![[r#"
foo
├── bar
└── baz

"#]]
    );
}

#[test]
fn render_tree_with_multiline_leaf() {
    let tree = Tree::new("foo").with_leaves([
        Tree::new("hello\nworld").with_multiline(true),
        Tree::new("goodbye\nworld").with_multiline(true),
    ]);
    assert_data_eq!(
        format!("{}", tree),
        str![[r#"
foo
├── hello
│   world
└── goodbye
    world

"#]]
    );
}
