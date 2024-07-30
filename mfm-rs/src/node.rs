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

fn quote(child: Vec<MfmNode>) -> NodeType {
    NodeType::Quote({
        children: child,
    })
}

// Implement traits to MfmQuote and continue from MfmSearch
// src: gh::raspberry-garage/mfm-js::develop::src/node
//impl 

const BLOCK_TYPES: [&str; 5] = ["quote", "search", "blockCode", "mathBlock", "center"];

pub fn is_mfm_block(node: &MfmNode) -> bool {
    BLOCK_TYPES.contains(&node.node_type.as_str())
}