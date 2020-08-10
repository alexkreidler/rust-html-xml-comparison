use std::default::Default;
use std::io::{self, Write};

use html5ever::driver::ParseOpts;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};

use markup5ever::{local_name, namespace_prefix, namespace_url, ns};
use std::iter::repeat;
// use std::str::escape_default;
// fn escape_default(s: &str) -> String {
//     s.chars().flat_map(|c| c.escape_default()).collect()
// }

fn walk(indent: usize, handle: &Handle) {
    let node = handle;
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.data {
        NodeData::Document => println!("#Document"),

        NodeData::Doctype {
            ref name,
            ref public_id,
            ref system_id,
        } => println!("<!DOCTYPE {} \"{}\" \"{}\">", name, public_id, system_id),

        NodeData::Text { ref contents } => {
            println!("#text: {}", &contents.borrow().escape_default())
        }

        NodeData::Comment { ref contents } => println!("<!-- {} -->", contents.escape_default()),

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.borrow().iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }

        NodeData::ProcessingInstruction { .. } => unreachable!(),
    }

    for child in node.children.borrow().iter() {
        walk(indent + 4, child);
    }
}
fn main() {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let html = hxcmp::data::invalid;

    let mut inp = html.as_bytes();

    // let inp = std::io::Read::new(html);

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut inp)
        .unwrap();

    walk(0, &dom.document);
}
