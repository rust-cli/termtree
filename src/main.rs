extern crate treeline;

use treeline::Tree;

fn main() {
    let b = Tree(
        "root",
        vec![
            Tree(
                "a",
                vec![
                    Tree(
                        "c",
                        vec![
                            Tree("d", vec![])
                         ]
                    )
                ]
            ),
            Tree("b", vec![])
        ]
    );
    println!("{}", b.render())
}
