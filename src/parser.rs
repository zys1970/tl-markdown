use crate::ast::AstNode;
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};

#[derive(Debug)]
enum Frame {
    Root,
    Heading { level: u8 },
    Paragraph,
    Emphasis,
    Strong,
    BlockQuote,
    List { ordered: bool },
    ListItem,
    Link { url: String },
    CodeBlock { language: Option<String> },
}

struct StackItem {
    frame: Frame,
    children: Vec<AstNode>,
}

pub fn parse_markdown(input: &str) -> AstNode {
    let parser = Parser::new(input);

    let mut stack: Vec<StackItem> = vec![StackItem {
        frame: Frame::Root,
        children: Vec::new(),
    }];

    for event in parser {
        match event {
            Event::Start(tag) => {
                stack.push(StackItem {
                    frame: tag_to_frame(tag),
                    children: Vec::new(),
                });
            }

            Event::End(_) => {
                let item = stack.pop().unwrap();
                if let Some(node) = build_node(item.frame, item.children) {
                    stack.last_mut().unwrap().children.push(node);
                }
            }

            Event::Text(text) => {
                stack
                    .last_mut()
                    .unwrap()
                    .children
                    .push(AstNode::Text{value: text.to_string()});
            }

            Event::Code(code) => {
                stack
                    .last_mut()
                    .unwrap()
                    .children
                    .push(AstNode::InlineCode{value: code.to_string()});
            }

            _ => {}
        }
    }

    let root = stack.pop().unwrap();
    build_node(root.frame, root.children).unwrap()
}

fn tag_to_frame(tag: Tag) -> Frame {
    match tag {
        Tag::Heading { level, .. } => Frame::Heading { level: level as u8 },

        Tag::Paragraph => Frame::Paragraph,

        Tag::Emphasis => Frame::Emphasis,
        Tag::Strong => Frame::Strong,

        Tag::BlockQuote => Frame::BlockQuote,

        Tag::List(start) => Frame::List {
            ordered: start.is_some(),
        },

        Tag::Item => Frame::ListItem,

        Tag::Link { dest_url, .. } => Frame::Link {
            url: dest_url.to_string(),
        },

        Tag::CodeBlock(kind) => {
            let lang = match kind {
                CodeBlockKind::Fenced(l) => Some(l.to_string()),
                CodeBlockKind::Indented => None,
            };
            Frame::CodeBlock { language: lang }
        }

        _ => Frame::Paragraph,
    }
}

fn build_node(frame: Frame, children: Vec<AstNode>) -> Option<AstNode> {
    match frame {
        Frame::Root => Some(AstNode::Document{children}),

        Frame::Heading { level } => Some(AstNode::Heading { level, children }),

        Frame::Paragraph => Some(AstNode::Paragraph{children}),

        Frame::Emphasis => Some(AstNode::Emphasis{children}),
        Frame::Strong => Some(AstNode::Strong{children}),

        Frame::BlockQuote => Some(AstNode::BlockQuote{children}),

        Frame::List { ordered } => {
            let mut items = Vec::new();
            for node in children {
                if let AstNode::ListItem{children: item} = node {
                    items.push(item);
                }
            }
            Some(AstNode::List { ordered, items })
        }

        Frame::ListItem => Some(AstNode::ListItem { children }),

        Frame::Link { url } => Some(AstNode::Link { url, children }),

        Frame::CodeBlock { language } => {
            let mut code = String::new();
            for node in children {
                if let AstNode::Text{value:t} = node {
                    code.push_str(&t);
                }
            }
            Some(AstNode::CodeBlock { language, code })
        }
    }
}
