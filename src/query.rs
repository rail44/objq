use super::Value;
use rustc_serialize;

peg! grammer(r#"
use super::{Query, Selector};

#[pub]
parse -> Query
    = selectors:selector+ { Query { selectors: selectors } }

selector -> Selector
    = "." k:key { Selector::Key(k) }
    / "[" i:index "]" { Selector::Index(i) }

key -> String
    = [^\.\[]* { match_str.to_owned() }

index -> usize
    = [0-9]+ { match_str.parse().unwrap() }
"#);

#[derive(Debug)]
pub struct Query {
    selectors: Vec<Selector>,
}

impl Query {
    pub fn selectors<'a>(&'a self) -> &'a [Selector] {
        self.selectors.as_slice()
    }
}

#[derive(Debug)]
pub enum Selector {
    Key(String),
    Index(usize),
}

pub trait Queryable {
    fn query<'a>(&'a self, query: &Query) -> Option<&'a Self> {
        let mut current = self;
        for selector in query.selectors() {
            match current.select(selector) {
                Some(v) => { current = v },
                None => return None,
            }
        }
        Some(current)
    }

    fn select<'a>(&'a self, selector: &Selector) -> Option<&'a Self>;
}

impl Queryable for Value {
    fn select<'a>(&'a self, selector: &Selector) -> Option<&'a Value> {
        match selector {
            &Selector::Key(ref k) => self.find(k.as_str()),
            &Selector::Index(ref i) => self.as_array().and_then(|array| array.get(i.clone())),
        }
    }
}

impl rustc_serialize::Decodable for Query {
    fn decode<D: rustc_serialize::Decoder>(d: &mut D) -> Result<Query, D::Error> {
        d.read_str().and_then(|string| {
            grammer::parse(string.as_str()).map_err(|e| {
                d.error(format!("{:?}", e).as_str())
            })
        })
    }
}
