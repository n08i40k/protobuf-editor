use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub proto3);

pub mod ast;
pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::{lexer, proto3};

    macro_rules! parse_ast {
        ($file:literal) => {{
            let data = include_str!(concat!("../../../test-data/proto-parser/", $file));

            let lexer = lexer::Lexer::new(&data);
            let parser = proto3::GlobalScopeParser::new();

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
        let target_ast = vec![ast::Expr::Syntax("proto3")];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn package_simple() {
        let ast = parse_ast!("package-simple.proto");
        let target_ast = vec![ast::Expr::Syntax("proto3"), ast::Expr::Package("mypkg")];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn package_complex() {
        let ast = parse_ast!("package-complex.proto");
        let target_ast = vec![ast::Expr::Syntax("proto3"), ast::Expr::Package("my.pkg")];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn import() {
        let ast = parse_ast!("import.proto");
        let target_ast = vec![
            ast::Expr::Syntax("proto3"),
            ast::Expr::Import("google/protobuf/any.proto"),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn message_empty() {
        let ast = parse_ast!("message-empty.proto");
        let target_ast = vec![
            ast::Expr::Syntax("proto3"),
            ast::Expr::Message(ast::Message {
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
            ast::Expr::Syntax("proto3"),
            ast::Expr::Message(ast::Message {
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
            ast::Expr::Syntax("proto3"),
            ast::Expr::Message(ast::Message {
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
            ast::Expr::Syntax("proto3"),
            ast::Expr::Enum(ast::Enum {
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
            ast::Expr::Syntax("proto3"),
            ast::Expr::Import("google/protobuf/descriptor.proto"),
            ast::Expr::Option(ast::Option {
                ident: "java_multiple_files",
                value: ast::OptionValue::Boolean(true),
            }),
            ast::Expr::Option(ast::Option {
                ident: "java_package",
                value: ast::OptionValue::String("xd.xd"),
            }),
            ast::Expr::Extend(ast::Extend {
                r#type: "google.protobuf.EnumValueOptions",
                entries: vec![ast::ExtendEntry::Field(ast::Field {
                    modifier: ast::FieldModifier::Optional,
                    r#type: "bool",
                    ident: "own_enum_value",
                    index: 2000,
                    options: vec![],
                })],
            }),
            ast::Expr::Extend(ast::Extend {
                r#type: "google.protobuf.FieldOptions",
                entries: vec![ast::ExtendEntry::Field(ast::Field {
                    modifier: ast::FieldModifier::Optional,
                    r#type: "bool",
                    ident: "own_field_value",
                    index: 2000,
                    options: vec![ast::Option {
                        ident: "deprecated",
                        value: ast::OptionValue::Boolean(true),
                    }],
                })],
            }),
            ast::Expr::Enum(ast::Enum {
                ident: "Enum",
                entries: vec![
                    ast::EnumEntry::Option(ast::Option {
                        ident: "allow_alias",
                        value: ast::OptionValue::Boolean(true),
                    }),
                    ast::EnumEntry::Pair {
                        ident: "FIRST",
                        value: 0,
                        options: vec![ast::Option {
                            ident: "deprecated",
                            value: ast::OptionValue::Boolean(true),
                        }],
                    },
                    ast::EnumEntry::Pair {
                        ident: "SECOND",
                        value: 0,
                        options: vec![ast::Option {
                            ident: "(own_enum_value)",
                            value: ast::OptionValue::Boolean(true),
                        }],
                    },
                ],
            }),
            ast::Expr::Message(ast::Message {
                ident: "Message",
                entries: vec![
                    ast::MessageEntry::Option(ast::Option {
                        ident: "deprecated",
                        value: ast::OptionValue::Boolean(true),
                    }),
                    ast::MessageEntry::Field(ast::Field {
                        modifier: ast::FieldModifier::None,
                        r#type: "bool",
                        ident: "var",
                        index: 1,
                        options: vec![
                            ast::Option {
                                ident: "deprecated",
                                value: ast::OptionValue::Boolean(true),
                            },
                            ast::Option {
                                ident: "(own_field_value)",
                                value: ast::OptionValue::Boolean(false),
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
            ast::Expr::Syntax("proto3"),
            ast::Expr::Import("google/protobuf/descriptor.proto"),
            ast::Expr::Comment(ast::Comment::single_line("// single line comment")),
            ast::Expr::Comment(ast::Comment::single_line("// another single line comment")),
            ast::Expr::Comment(ast::Comment::multi_line("/* multi\n   line\n   comment */")),
            ast::Expr::Message(ast::Message {
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
            ast::Expr::Enum(ast::Enum {
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
            ast::Expr::Extend(ast::Extend {
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
            ast::Expr::Comment(ast::Comment::single_line("// at the bottom of the file")),
        ];

        assert_eq!(ast, target_ast);
    }

    #[test]
    fn extensions() {
        let ast = parse_ast!("extensions.proto");
        let target_ast = vec![
            ast::Expr::Syntax("proto2"),
            ast::Expr::Message(ast::Message {
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
}
