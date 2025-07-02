use std::ops::Range;

pub type GlobalScope<'a> = Vec<Expr<'a>>;

#[derive(Debug, PartialEq)]
pub struct Option<'a> {
    pub ident: &'a str,
    pub value: OptionValue<'a>,
}

#[derive(Debug, PartialEq)]
pub enum OptionValue<'a> {
    Boolean(bool),
    Integer(i64),
    Ident(&'a str),
    String(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct Comment<'a> {
    pub r#type: CommentType,
    pub source: Vec<&'a str>,
    pub text: String,
}

impl<'a> Comment<'a> {
    pub fn single_line(text: &'a str) -> Self {
        println!("sl: {:?}", text);

        Self {
            r#type: CommentType::SingleLine,
            text: text.to_string(),
            source: vec![&text[2..]],
        }
    }

    pub fn multi_line(text: &'a str) -> Self {
        println!("ml: {:?}", text);

        Self {
            r#type: CommentType::MultiLine,
            text: text.to_string(),
            source: vec![&text[2..text.len() - 2]],
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

#[derive(Debug, PartialEq)]
pub enum MessageEntry<'a> {
    Option(Option<'a>),

    Field(Field<'a>),
    Message(Message<'a>),
    Extend(Extend<'a>),
    Enum(Enum<'a>),

    ReservedIndices(Vec<Range<i64>>),
    ReservedIdents(Vec<&'a str>),
}

#[derive(Debug, PartialEq)]
pub struct Field<'a> {
    pub modifier: FieldModifier,
    pub r#type: &'a str,
    pub ident: &'a str,
    pub index: i64,
    pub options: Vec<Option<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum FieldModifier {
    None,
    Optional,
    Repeated,
}

/// Extend
#[derive(Debug, PartialEq)]
pub struct Extend<'a> {
    pub r#type: &'a str,
    pub fields: Vec<Field<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Enum<'a> {
    pub ident: &'a str,
    pub entries: Vec<EnumEntry<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum EnumEntry<'a> {
    Option(Option<'a>),
    Pair {
        ident: &'a str,
        value: i64,
        options: Vec<Option<'a>>,
    },
}
