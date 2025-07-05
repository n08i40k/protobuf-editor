use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    #[allow(clippy::ptr_arg)]
    #[rustfmt::skip]
    pub proto
);

pub mod ast;
pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::{lexer, proto};

    macro_rules! parse_ast {
        ($file:literal) => {{
            let data = include_str!(concat!("../../../test-data/proto-parser/", $file));

            let lexer = lexer::Lexer::new(&data);
            let parser = proto::FileParser::new();

            match parser.parse(data, lexer) {
                Err(error) => panic!("{}", error),
                Ok(ast) => ast,
            }
        }};
    }

    #[test]
    fn empty() {
        let ast = parse_ast!("empty.proto");
        assert!(ast.is_empty());
    }

    #[test]
    fn syntax() {
        let ast = parse_ast!("syntax.proto");
        let target_ast = vec![ast::FileEntry::Syntax("proto3")];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn package_simple() {
        let ast = parse_ast!("package-simple.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Package("mypkg"),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn package_complex() {
        let ast = parse_ast!("package-complex.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Package("my.pkg"),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn import() {
        let ast = parse_ast!("import.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Import("google/protobuf/any.proto"),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn message_empty() {
        let ast = parse_ast!("message-empty.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Message(ast::Message {
                ident: "Empty",
                entries: vec![],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn message() {
        let ast = parse_ast!("message.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Message(ast::Message {
                ident: "Message",
                entries: vec![
                    ast::MessageEntry::ReservedIndices(vec![
                        ast::Range::from(2),
                        ast::Range::from((6, ())),
                    ]),
                    ast::MessageEntry::ReservedIdents(vec!["sample"]),
                    ast::MessageEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::None,
                        r#type: "bool",
                        ident: "first",
                        index: 1,
                        options: vec![],
                    }),
                    ast::MessageEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::Optional,
                        r#type: "string",
                        ident: "third",
                        index: 3,
                        options: vec![],
                    }),
                    ast::MessageEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::Repeated,
                        r#type: "uint64",
                        ident: "fourth",
                        index: 4,
                        options: vec![],
                    }),
                    ast::MessageEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::None,
                        r#type: "map<string, string>",
                        ident: "fifth",
                        index: 5,
                        options: vec![],
                    }),
                ],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn message_inner() {
        let ast = parse_ast!("message-inner.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Message(ast::Message {
                ident: "Parent",
                entries: vec![
                    ast::MessageEntry::Message(ast::Message {
                        ident: "Child",
                        entries: vec![ast::MessageEntry::Field(ast::Field {
                            modifier: ast::FieldModifier::None,
                            r#type: "bool",
                            ident: "var",
                            index: 1,
                            options: vec![],
                        })],
                    }),
                    ast::MessageEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::None,
                        r#type: "Child",
                        ident: "child",
                        index: 1,
                        options: vec![],
                    }),
                ],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn r#enum() {
        let ast = parse_ast!("enum.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Enum(ast::Enum {
                ident: "Enum",
                entries: vec![
                    ast::EnumEntry::Pair {
                        ident: "ZERO",
                        value: 0,
                        options: vec![],
                    },
                    ast::EnumEntry::Pair {
                        ident: "POSITIVE",
                        value: 1,
                        options: vec![],
                    },
                    ast::EnumEntry::Pair {
                        ident: "NEGATIVE",
                        value: -1,
                        options: vec![],
                    },
                ],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn options() {
        let ast = parse_ast!("options.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Import("google/protobuf/descriptor.proto"),
            ast::FileEntry::Option(ast::Option {
                key: "java_multiple_files",
                value: ast::MapValue::Boolean(true),
            }),
            ast::FileEntry::Option(ast::Option {
                key: "java_package",
                value: ast::MapValue::String("xd.xd"),
            }),
            ast::FileEntry::Extend(ast::Extend {
                r#type: "google.protobuf.EnumValueOptions",
                entries: vec![ast::ExtendEntry::Field(ast::Field {
                    modifier: ast::FieldModifier::Optional,
                    r#type: "bool",
                    ident: "own_enum_value",
                    index: 2000,
                    options: vec![],
                })],
            }),
            ast::FileEntry::Extend(ast::Extend {
                r#type: "google.protobuf.FieldOptions",
                entries: vec![ast::ExtendEntry::Field(ast::Field {
                    modifier: ast::FieldModifier::Optional,
                    r#type: "bool",
                    ident: "own_field_value",
                    index: 2000,
                    options: vec![ast::Option {
                        key: "deprecated",
                        value: ast::MapValue::Boolean(true),
                    }],
                })],
            }),
            ast::FileEntry::Enum(ast::Enum {
                ident: "Enum",
                entries: vec![
                    ast::EnumEntry::Option(ast::Option {
                        key: "allow_alias",
                        value: ast::MapValue::Boolean(true),
                    }),
                    ast::EnumEntry::Pair {
                        ident: "FIRST",
                        value: 0,
                        options: vec![ast::Option {
                            key: "deprecated",
                            value: ast::MapValue::Boolean(true),
                        }],
                    },
                    ast::EnumEntry::Pair {
                        ident: "SECOND",
                        value: 0,
                        options: vec![ast::Option {
                            key: "(own_enum_value)",
                            value: ast::MapValue::Boolean(true),
                        }],
                    },
                ],
            }),
            ast::FileEntry::Message(ast::Message {
                ident: "Message",
                entries: vec![
                    ast::MessageEntry::Option(ast::Option {
                        key: "deprecated",
                        value: ast::MapValue::Boolean(true),
                    }),
                    ast::MessageEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::Optional,
                        r#type: "bool",
                        ident: "var",
                        index: 1,
                        options: vec![
                            ast::Option {
                                key: "deprecated",
                                value: ast::MapValue::Boolean(true),
                            },
                            ast::Option {
                                key: "(own_field_value)",
                                value: ast::MapValue::Boolean(false),
                            },
                            ast::Option {
                                key: "edition_defaults",
                                value: ast::MapValue::Map(ast::JSONLikeMap::from([
                                    ("edition", ast::MapValue::Ident("EDITION_PROTO2")),
                                    ("value", ast::MapValue::String("true")),
                                ])),
                            },
                            ast::Option {
                                key: "edition_defaults",
                                value: ast::MapValue::Map(ast::JSONLikeMap::from([
                                    ("edition", ast::MapValue::Ident("EDITION_PROTO3")),
                                    ("value", ast::MapValue::String("false")),
                                ])),
                            },
                        ],
                    }),
                ],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn comments() {
        let ast = parse_ast!("comments.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Import("google/protobuf/descriptor.proto"),
            ast::FileEntry::Comment(ast::Comment::single_line("// single line comment")),
            ast::FileEntry::Comment(ast::Comment::single_line("// another single line comment")),
            ast::FileEntry::Comment(ast::Comment::multi_line("/* multi\n   line\n   comment */")),
            ast::FileEntry::Message(ast::Message {
                ident: "Message",
                entries: vec![
                    ast::MessageEntry::Comment(ast::Comment::single_line("// in message")),
                    ast::MessageEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::None,
                        r#type: "bool",
                        ident: "var",
                        index: 1,
                        options: vec![],
                    }),
                    ast::MessageEntry::Comment(ast::Comment::single_line("// right after entry")),
                    ast::MessageEntry::Comment(ast::Comment::single_line("// at the bottom")),
                ],
            }),
            ast::FileEntry::Enum(ast::Enum {
                ident: "Enum",
                entries: vec![
                    ast::EnumEntry::Comment(ast::Comment::single_line("// in enum")),
                    ast::EnumEntry::Pair {
                        ident: "DEFAULT",
                        value: 0,
                        options: vec![],
                    },
                ],
            }),
            ast::FileEntry::Extend(ast::Extend {
                r#type: "google.protobuf.FieldOptions",
                entries: vec![
                    ast::ExtendEntry::Comment(ast::Comment::single_line("// in extend")),
                    ast::ExtendEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::Optional,
                        r#type: "bool",
                        ident: "var",
                        index: 1,
                        options: vec![],
                    }),
                ],
            }),
            ast::FileEntry::Comment(ast::Comment::single_line("// at the bottom of the file")),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn extensions() {
        let ast = parse_ast!("extensions.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto2"),
            ast::FileEntry::Message(ast::Message {
                ident: "Message",
                entries: vec![ast::MessageEntry::Extensions(vec![
                    ast::Range::from(1),
                    ast::Range::from(2..5),
                    ast::Range::from((6, ())),
                ])],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn required() {
        let ast = parse_ast!("required.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto2"),
            ast::FileEntry::Message(ast::Message {
                ident: "Message",
                entries: vec![ast::MessageEntry::Field(ast::Field {
                    modifier: ast::FieldModifier::Required,
                    r#type: "bool",
                    ident: "var",
                    index: 1,
                    options: vec![],
                })],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn keywords() {
        let ast = parse_ast!("keywords.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Message(ast::Message::empty("Ident")),
            ast::FileEntry::Message(ast::Message {
                ident: "to",
                entries: vec![ast::MessageEntry::Message(ast::Message::empty("inner"))],
            }),
            ast::FileEntry::Message(ast::Message::empty("max")),
            ast::FileEntry::Message(ast::Message::empty("syntax")),
            ast::FileEntry::Message(ast::Message::empty("option")),
            ast::FileEntry::Message(ast::Message::empty("package")),
            ast::FileEntry::Message(ast::Message::empty("import")),
            ast::FileEntry::Message(ast::Message::empty("message")),
            ast::FileEntry::Message(ast::Message::empty("oneof")),
            ast::FileEntry::Message(ast::Message::empty("extend")),
            ast::FileEntry::Message(ast::Message::empty("enum")),
            ast::FileEntry::Message(ast::Message::empty("reserved")),
            ast::FileEntry::Message(ast::Message::empty("extensions")),
            ast::FileEntry::Message(ast::Message::empty("optional")),
            ast::FileEntry::Message(ast::Message::empty("required")),
            ast::FileEntry::Message(ast::Message::empty("repeated")),
            ast::FileEntry::Message(ast::Message::empty("map")),
            ast::FileEntry::Message(ast::Message {
                ident: "Message",
                entries: vec![
                    ast::MessageEntry::Field(ast::Field::basic("bool", "var1", 1)),
                    ast::MessageEntry::Field(ast::Field::basic("Ident", "var2", 2)),
                    ast::MessageEntry::Field(ast::Field::basic("to", "var3", 3)),
                    ast::MessageEntry::Field(ast::Field::basic("to.inner", "var4", 4)),
                    ast::MessageEntry::Field(ast::Field::basic("max", "var5", 5)),
                    ast::MessageEntry::Field(ast::Field::basic("syntax", "var6", 6)),
                    ast::MessageEntry::Field(ast::Field::basic("package", "var7", 7)),
                    ast::MessageEntry::Field(ast::Field::basic("import", "var8", 8)),
                ],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn oneof() {
        let ast = parse_ast!("oneof.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Message(ast::Message {
                ident: "Message",
                entries: vec![
                    ast::MessageEntry::OneOf(ast::OneOf {
                        ident: "OneOf",
                        entries: vec![
                            ast::OneOfEntry::Option(ast::Option {
                                key: "uninterpreted_option",
                                value: ast::MapValue::Map(ast::JSONLikeMap::from([(
                                    "string_value",
                                    ast::MapValue::String(""),
                                )])),
                            }),
                            ast::OneOfEntry::Field(ast::Field::basic("bool", "oneof_var", 1)),
                        ],
                    }),
                    ast::MessageEntry::Field(ast::Field::basic("bool", "message_var", 2)),
                ],
            }),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn service() {
        let ast = parse_ast!("service.proto");
        let target_ast = vec![
            ast::FileEntry::Syntax("proto3"),
            ast::FileEntry::Service(ast::Service {
                ident: "Service",
                entries: vec![
                    ast::ServiceEntry::Option(ast::Option {
                        key: "uninterpreted_option",
                        value: ast::MapValue::Map(ast::JSONLikeMap::from([(
                            "string_value",
                            ast::MapValue::String(""),
                        )])),
                    }),
                    ast::ServiceEntry::Rpc(ast::Rpc {
                        ident: "RPC1",
                        request: "Request",
                        reply: "Reply",
                        stream: ast::RpcStream::None,
                    }),
                    ast::ServiceEntry::Rpc(ast::Rpc {
                        ident: "RPC2",
                        request: "Request",
                        reply: "Reply",
                        stream: ast::RpcStream::ServerBound,
                    }),
                    ast::ServiceEntry::Rpc(ast::Rpc {
                        ident: "RPC3",
                        request: "Request",
                        reply: "Reply",
                        stream: ast::RpcStream::ClientBound,
                    }),
                    ast::ServiceEntry::Rpc(ast::Rpc {
                        ident: "RPC4",
                        request: "Request",
                        reply: "Reply",
                        stream: ast::RpcStream::Bidirectional,
                    }),
                ],
            }),
            ast::FileEntry::Message(ast::Message::empty("Request")),
            ast::FileEntry::Message(ast::Message::empty("Reply")),
        ];

        assert_eq!(ast, target_ast);
    }
}
