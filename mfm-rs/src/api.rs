mod super::internal;
use internal::{ full_parser, simple_parser, util };
use util::{ inspect_one, stringify_node, stringify_tree };

mod super::node;
use node::{ MfmNode, MfmSimpleNode };

use std::collections::HashMap;

fn parse(input: &str, opts: Option<HashMap<String, Option<usize>>>) -> Vec<MfmNode> {
    let mut options = HashMap::new();
    if let Some(opts) = opts {
        options = opts;
    }
    
    let nodes = full_parser(input, &options);
    nodes
}

