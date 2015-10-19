#[derive(Debug)]
pub struct Tree<'a>(
   pub  &'a str,
   pub Vec<Tree<'a>>
);

impl<'a> Tree<'a> {
    pub fn render(&self) -> String {
        fn with_padding(b: &Tree, pad: &str) -> String {
            format!(
                "{}\n{}{} ",
                &b.0.clone(),
                pad,
                if b.1.is_empty() { ' ' } else { '│' }
            )
        }

        fn recurse(b: &Tree, pad: &str) -> String {
            let mut s = String::new();
            s.push_str(pad);
            s.push_str(&with_padding(&b, pad));
            s.push('\n');
            let tail = b.1.iter().enumerate()
                .fold(String::new(), |mut out, (i, b2)| {
                    let last = i == b.1.len() - 1;
                    let more = !b2.1.is_empty();
                    let next_pad = format!(
                        "{}{} ", pad, if last { ' ' } else { '│' }
                    );
                    let next = recurse(&b2, &next_pad);
                    out.push_str(pad);
                    out.push(if last { '└' } else { '├' });
                    out.push('─');
                    out.push(if more {'┬'} else {'─'});
                    out.push(' ');
                    out.push_str(&next[next_pad.len()..next.len()]);
                    out
                }
            );
            s.push_str(&tail);
            s
        }
        recurse(&self, "")
    }
}

#[test]
fn it_works() {
}
