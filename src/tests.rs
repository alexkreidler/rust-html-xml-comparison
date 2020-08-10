pub struct Test<'a> {
    pub name: &'a str,
    pub input: Input<'a>,
}

// #[derive(Clone, Debug)]
pub enum Input<'a> {
    Static(&'static str),
    AString(String),
    Reader(Box<dyn std::io::BufRead + 'a>),
}

pub type Tests<'a> = Vec<Test<'a>>;

use crate::data;

pub fn get_tests() -> Tests<'static> {
    return vec![
        Test {
            name: "basic",
            input: Input::Static(data::basic),
        },
        Test {
            name: "basic2",
            input: Input::Static(data::basic2),
        },
        Test {
            name: "invalid",
            input: Input::Static(data::invalid),
        },
        Test {
            name: "invalid2",
            input: Input::Static(data::invalid2),
        },
        Test {
            name: "invalid3",
            input: Input::Static(data::invalid3),
        },
        Test {
            name: "self_closed",
            input: Input::Static(data::self_closed),
        },
    ];
}
