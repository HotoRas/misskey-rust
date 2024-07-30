mod api;

pub use api::{
    parse,
    parse_simple,
    to_string,
    inspect,
    extract
};

mod node;

pub use node::{
    node_type,
    mfm_node,
    mfm_simple_node,
    mfm_block,
    mfm_inline,
};

pub use node::{
    mfm_quote,
    mfm_search,
    mfm_code_block,
    mfm_math_block,
    mfm_center,

    mfm_unicode_emoji,
    mfm_emoji_code,
    mfm_bold,
    mfm_small,
    mfm_italic,
    mfm_strike,
    mfm_inline_code,
    mfm_math_inline,
    mfm_mention,
    mfm_hashtag,
    mfm_url,
    mfm_link,
    mfm_fn,
    mfm_plain,
    mfm_text,
};

pub use node::{
	// block
	QUOTE,
	SEARCH,
	CODE_BLOCK,
	MATH_BLOCK,
	CENTER,

	// inline
	UNI_EMOJI,
	EMOJI_CODE,
	BOLD,
	SMALL,
	ITALIC,
	STRIKE,
	INLINE_CODE,
	MATH_INLINE,
	MENTION,
	HASHTAG,
	N_URL,
	LINK,
	FN,
	PLAIN,
	TEXT,
}