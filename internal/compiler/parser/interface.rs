// Copyright © 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>, author Nathan Collins <nathan.collins@kdab.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-2.0 OR LicenseRef-Slint-Software-3.0

//! Module containing the parsing functions for interfaces

use super::prelude::*;

#[cfg_attr(test, parser_test)]
/// ```test,InterfaceDeclaration
/// interface I { }
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
    p.expect(SyntaxKind::RBrace)
}
