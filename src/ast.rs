use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum AstNode {
    // 根节点
    Document {
        children: Vec<AstNode>,
    },
    Heading {
        level: u8,
        children: Vec<AstNode>,
    },
    Paragraph {
        children: Vec<AstNode>,
    },
    BlockQuote {
        children: Vec<AstNode>,
    },
    List {
        ordered: bool,
        items: Vec<Vec<AstNode>>,
    },
    ListItem {
        children: Vec<AstNode>,
    },
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    Table {
        headers: Vec<AstNode>,
        rows: Vec<Vec<AstNode>>,
    },
    Text{
        value: String,
    },
    Strong{
        children: Vec<AstNode>
    },   // **bold**
    Emphasis{
        children: Vec<AstNode>
    }, // *italic*
    InlineCode{
        value: String
    },
    Link {
        url: String,
        children: Vec<AstNode>,
    },
    Image {
        url: String,
        alt: Option<String>,
    },
    HorizontalRule,
    Footnote {
        name: String,
        children: Vec<AstNode>,
    },
    Html{
        value: String
    }, // 保留原生 HTML
}
