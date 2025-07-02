use std::collections::HashMap;

pub type GlobalScope<'a> = Vec<Expr<'a>>;

#[derive(Debug, PartialEq)]
pub enum RangeEnd {
    Integer(i64),
    Max,
}

#[derive(Debug, PartialEq)]
pub struct Range {
    pub start: i64,
    pub end: RangeEnd,
}

impl From<i64> for Range {
    fn from(start: i64) -> Self {
        Self {
            start,
            end: RangeEnd::Integer(start + 1),
        }
    }
}

impl From<std::ops::Range<i64>> for Range {
    fn from(range: std::ops::Range<i64>) -> Self {
        Self {
            start: range.start,
            end: RangeEnd::Integer(range.end + 1),
        }
    }
}

impl From<(i64, ())> for Range {
    fn from(range: (i64, ())) -> Self {
        Self {
            start: range.0,
            end: RangeEnd::Max,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MapValue<'a> {
    Boolean(bool),
    Integer(i64),
    Ident(&'a str),
    String(&'a str),
    Map(JSONLikeMap<'a>),
}

pub type JSONLikeMap<'a> = HashMap<&'a str, MapValue<'a>>;

#[derive(Debug, PartialEq)]
pub struct Option<'a> {
    pub key: &'a str,
    pub value: MapValue<'a>,
}

#[derive(Debug, PartialEq)]
pub struct Comment<'a> {
    pub r#type: CommentType,
    pub source: &'a str,
    pub text: &'a str,
}

impl<'a> Comment<'a> {
    pub fn single_line(source: &'a str) -> Self {
        Self {
            r#type: CommentType::SingleLine,
            text: source[2..].trim(),
            source,
        }
    }

    pub fn multi_line(source: &'a str) -> Self {
        Self {
            r#type: CommentType::MultiLine,
            text: source[2..source.len() - 2].trim(),
            source,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum CommentType {
    SingleLine,
    MultiLine,
}

#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Comment(Comment<'a>),
    Syntax(&'a str),
    Package(&'a str),
    Import(&'a str),
    Option(Option<'a>),
    Message(Message<'a>),
    Extend(Extend<'a>),
    Enum(Enum<'a>),
}

/// Message
#[derive(Debug, PartialEq)]
pub struct Message<'a> {
    pub ident: &'a str,
    pub entries: Vec<MessageEntry<'a>>,
}

impl<'a> Message<'a> {
    pub fn empty(name: &'a str) -> Self {
        Self {
            ident: name,
            entries: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MessageEntry<'a> {
    Comment(Comment<'a>),
    Option(Option<'a>),

    Field(Field<'a>),
    Message(Message<'a>),
    Extend(Extend<'a>),
    Enum(Enum<'a>),

    ReservedIndices(Vec<Range>),
    ReservedIdents(Vec<&'a str>),

    Extensions(Vec<Range>),
}

#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    pub modifier: FieldModifier,
    pub r#type: &'a str,
    pub ident: &'a str,
    pub index: i64,
    pub options: Vec<Option<'a>>,
}

impl<'a> Field<'a> {
    pub fn basic(r#type: &'a str, ident: &'a str, index: i64) -> Self {
        Self {
            modifier: FieldModifier::None,
            r#type,
            ident,
            index,
            options: vec![],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FieldModifier {
    None,
    Optional,
    Required,
    Repeated,
}

/// Extend
#[derive(Debug, PartialEq)]
pub struct Extend<'a> {
    pub r#type: &'a str,
    pub entries: Vec<ExtendEntry<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum ExtendEntry<'a> {
    Comment(Comment<'a>),
    Field(Field<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Enum<'a> {
    pub ident: &'a str,
    pub entries: Vec<EnumEntry<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum EnumEntry<'a> {
    Comment(Comment<'a>),
    Option(Option<'a>),
    Pair {
        ident: &'a str,
        value: i64,
        options: Vec<Option<'a>>,
    },
}
