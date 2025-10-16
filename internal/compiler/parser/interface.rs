// Copyright © 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>, author Nathan Collins <nathan.collins@kdab.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! Module containing the parsing functions for interfaces

use super::prelude::*;
use super::r#type::parse_type;

#[cfg_attr(test, parser_test)]
/// ```test,InterfaceDeclaration
/// interface I { }
/// interface I { in property<int> x; }
/// ```
pub fn parse_interface_declaration<P: Parser>(
    p: &mut P,
    checkpoint: Option<P::Checkpoint>,
) -> bool {
    debug_assert_eq!(p.peek().as_str(), "interface");
    let mut p = p.start_node_at(checkpoint, SyntaxKind::InterfaceDeclaration);
    p.consume(); // "interface"
    {
        let mut p: crate::parser::Node<'_, P> = p.start_node(SyntaxKind::DeclaredIdentifier);
        p.expect(SyntaxKind::Identifier);
    }

    p.expect(SyntaxKind::LBrace);
    p.consume();
    parse_interface_content(&mut *p);
    p.expect(SyntaxKind::RBrace)
}

#[cfg_attr(test, parser_test)]
/// ```test
/// in property <int> xxx;
/// property<int> foobar;
/// ```
fn parse_interface_content(p: &mut impl Parser) {
    let mut had_parse_error = false;

    loop {
        match p.nth(0).kind() {
            SyntaxKind::RBrace => return,
            SyntaxKind::Eof => return,
            SyntaxKind::Identifier => match p.nth(1).kind() {
                SyntaxKind::LAngle | SyntaxKind::Identifier if p.peek().as_str() == "property" => {
                    parse_interface_property_declaration(&mut *p);
                }
                SyntaxKind::Identifier
                    if p.nth(1).as_str() == "property"
                        && matches!(
                            p.peek().as_str(),
                            "in" | "out" | "in_out" | "in-out" | "private"
                        ) =>
                {
                    parse_interface_property_declaration(&mut *p);
                }
                _ => {
                    if !had_parse_error {
                        p.error("Parse error");
                        had_parse_error = true;
                    }
                    p.consume();
                }
            },
            _ => {
                if !had_parse_error {
                    p.error("Parse error");
                    had_parse_error = true;
                }
                p.consume();
            }
        }
    }
}

#[cfg_attr(test, parser_test)]
/// ```test,InterfacePropertyDeclaration
/// in property <int> xxx;
/// property<int> foobar;
/// ```
fn parse_interface_property_declaration(p: &mut impl Parser) {
    let checkpoint = p.checkpoint();
    if p.peek().as_str() == "property" {
        p.warning("'private' properties are inaccessible in an interface");
    }
    while matches!(p.peek().as_str(), "in" | "out" | "in-out" | "in_out") {
        p.consume();
    }
    if p.peek().as_str() == "private" {
        p.warning("'private' properties are inaccessible in an interface");
        p.consume();
    } else if p.peek().as_str() != "property" {
        p.error("Expected 'property' keyword");
        return;
    }
    let mut p = p.start_node_at(checkpoint, SyntaxKind::InterfacePropertyDeclaration);
    p.consume(); // property

    if p.test(SyntaxKind::LAngle) {
        parse_type(&mut *p);
        p.expect(SyntaxKind::RAngle);
    } else if p.nth(0).kind() == SyntaxKind::Identifier {
        p.error("Missing type. The syntax to declare a property is `property <type> name;`. Two way bindings are not supported in an interface");
    }

    {
        let mut p = p.start_node(SyntaxKind::DeclaredIdentifier);
        p.expect(SyntaxKind::Identifier);
    }

    if p.peek().kind() == SyntaxKind::Colon {
        p.error("Property declarations in an interface cannot have default values");
    }
    p.expect(SyntaxKind::Semicolon);
}
