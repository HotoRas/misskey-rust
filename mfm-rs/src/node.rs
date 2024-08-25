#![feature(never_type)]

use std::collections::HashMap
use json::JsonValue

trait MfmNodeFn {
    pub fn get_type(&self) -> &str;
    pub fn get_props(&self) -> Option<HashMap<String, JsonValue>>;
}

#[derive(clone)]
enum MfmNode {
    Block(MfmBlock),
    Inline(MfmInline),
};

impl MfmNodeFn for MfmNode {
    pub fn get_type(&self) -> &str {
        match self {
            _(t) => t.get_type,
        }
    }
    pub fn get_props(&self) -> Option<HashMap<String, JsonValue>> {
        match self {
            _(t) => t.get_props,
        }
    }
}

#[derive(clone)]
enum MfmSimpleNode {
    UnicodeEmoji(MfmUnicodeEmoji),
    EmojiCode(MfmEmojiCode),
    Text(MfmText),
    Plain(MfmPlain),
};

impl MfmNodeFn for MfmSimpleNode {
    pub fn get_type(&self) -> &str {
        match self {
            _(t) => t.get_type,
        }
    }

    pub fn get_props(&self) -> Option<HashMap<String, JsonValue>> {
        match self {
            _(t) => t.get_props,
        }
    }
}

#[derive(clone)]
enum MfmBlock {
    Quote(MfmQuote),
    Search(MfmSearch),
    CodeBlock(MfmCodeBlock),
    MathBlock(MfmMathBlock),
    Center(MfmCenter),
};

impl MfmNodeFn for MfmBlock {
    pub fn get_type(&self) -> &str {
        match self {
            _(t) => t.get_type,
        }
    }

    pub fn get_props(&self) -> Option<HashMap<String, JsonValue>> {
        match self {
            _(t) => t.get_props,
        }
    }
}

trait MfmNodeType {
    pub const r#type: &str;
}

#[derive(clone)]
struct MfmQuote {
    props: Option<HashMap<String, JsonValue>>,
    children: Vec<MfmNode>,
}

impl MfmNodeType for MfmQuote {
    const r#type = "quote";
}

impl MfmNodeFn for MfmQuote {
    pub fn get_type(&self) -> &str { self::type }
    pub fn get_props(&self) -> Option<HashMap<String, JsonValue>> { self::props }
}

fn quote(child: Vec<MfmNode>) -> MfmNode {
    Block(Quote({
        children: child,
    }))
}

#[derive(clone)]
struct MfmSearch {
    props: MfmSearchProps,
    children: Option<Vec<MfmNode>>,
}

#[derive(clone)]
struct MfmSearchProps {
    query: String,
    content: String,
}

impl MfmNodeType for MfmQuote {
    const r#type = "search";
}

impl MfmNodeFn for MfmSearch {
    pub fn get_type(&self) -> &str { self::type }
    pub fn get_props(&self) -> Option<HashMap<String, JsonValue>> {
        Some({
            "query".to_string(): JsonValue::String(props.query),
            "content".to_string(): JsonValue::String(props.content),
        })
    }
}

impl MfmSearch {
    pub fn get_children(&self) -> Option<Vec<MfmNode>> { self::children }
}

fn search(query: String, content: String) -> MfmNode {
    Block(Search({
        props: {
            query: query,
            content: content,
        }
    }))
}

#[derive(clone)]
struct MfmMathBlock {
    props: MfmMathProps,
    children: Option<Vec<MfmNode>>,
}

#[derive(clone)]
struct MfmMathProps {
    formula: String,
}

impl MfmNodeType for MfmMathBlock {
    const r#type = "mathBlock";
}

impl MfmNodeFn for MfmMathBlock {
    pub fn get_type(&self) -> &str { self::type }
    pub fn get_props(&self) -> Option<HashMap<String, JsonValue>> {
        Some({
            "formula".to_string(): props.formula,
        })
    }
}

fn mathBlock(formula: String) -> MfmNode {
    Block(MathBlock({
        props: {
            formula: formula,
        },
    }))
}

struct MfmCenter {
    props: Option<HashMap<String, JsonValue>>,
    children: Vec<MfmInline>,
}

impl MfmNodeType for MfmCenter {
    const r#type = "center";
}

impl MfmNodeFn for MfmCenter {
    pub fn get_type(&self) -> &str {
        self::type
    }
    pub fn get_props(&self) -> Option<HashMap<String, JsonValue>> {
        self::props
    }
}

impl MfmCenter {
    pub fn get_children(&self) -> Vec<MfmInline> {
        self::children
    }
}
// continue from MfmInline
// src: gh::raspberry-garage/mfm-js::develop::src/node
//impl 

const BLOCK_TYPES: [&str; 5] = ["quote", "search", "blockCode", "mathBlock", "center"];

pub fn is_mfm_block(node: &MfmNode) -> bool {
    BLOCK_TYPES.contains(&node.node_type.as_str())
}