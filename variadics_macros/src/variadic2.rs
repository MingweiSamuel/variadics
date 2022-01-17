use std::iter::Peekable;

use proc_macro::{Delimiter, Group, Punct, Spacing, Span, TokenStream, TokenTree};

#[derive(Debug, Default)]
struct VariadicItem {
    pub spread_span: Option<Span>,
    pub stream: TokenStream,
    pub span: Option<Span>,
}

pub(crate) fn unit() -> TokenTree {
    TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new()))
}
pub(crate) fn comma() -> TokenTree {
    TokenTree::Punct(Punct::new(',', Spacing::Alone))
}

pub(crate) fn variadic(input: TokenStream) -> TokenStream {
    let mut items = Vec::new();

    let mut iter = itertools::peek_nth(input);
    while iter.peek().is_some() {
        let mut depth = 0_i32;
        let mut item = VariadicItem::default();

        // Check for `...spread` syntax.
        let spread = [Spacing::Joint, Spacing::Joint, Spacing::Alone]
            .iter()
            .enumerate()
            .all(|(i, &spacing)| {
                if let Some(TokenTree::Punct(punct)) = iter.peek_nth(i) {
                    '.' == punct.as_char() && spacing == punct.spacing()
                } else {
                    false
                }
            });
        if spread {
            item.spread_span = Some(
                iter.by_ref()
                    .take(3)
                    .map(|x| x.span())
                    .reduce(|a, b| a.join(b).unwrap())
                    .unwrap(),
            );
        }

        while let Some(token) = iter.next() {
            if let TokenTree::Punct(punct) = &token {
                match punct.as_char() {
                    ',' => {
                        if depth <= 0 {
                            break;
                        }
                    }
                    '<' => {
                        depth += 1;
                    }
                    '>' => {
                        depth -= 1;
                        if depth < 0 {
                            punct.span().error(
                                "Complex expressions in variadics must be grouped to avoid ambiguity.",
                            );
                        }
                    }
                    _ => {}
                }
            }
            item.span = item.span.and_then(|old_span| old_span.join(token.span()));
            item.stream.extend([token]);
        }
        items.push(item);
    }

    println!("ITEMS: {:#?}", items);

    fn helper(mut iter: Peekable<impl Iterator<Item = VariadicItem>>) -> TokenStream {
        match iter.next() {
            Some(VariadicItem {
                spread_span,
                mut stream,
                span,
            }) => {
                if let Some(spread_span) = spread_span {
                    if iter.peek().is_none() {
                        stream
                    }
                    else {
                        spread_span
                            .error("Spread elements are only supported in the final position of a variadic tuple type.")
                            .emit();
                        stream // TODO
                    }
                } else {
                    let recurse = helper(iter);
                    stream.extend([comma().into(), recurse]);
                    let mut group = Group::new(Delimiter::Parenthesis, stream);
                    if let Some(span) = span {
                        group.set_span(span);
                    }
                    TokenTree::Group(group).into()
                }
            }
            None => unit().into(),
        }
    }
    helper(items.into_iter().peekable())
}
