// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree

use super::{
    Comma, LeftParen, RightParen, whitespace::EmptyLine,
};

use std::ops::Deref;

pub trait WithComma {
    fn with_comma(self, comma: Comma) -> Self;
}

pub trait WithLeadingLines {
    fn leading_lines(&mut self) -> &mut Vec<EmptyLine>;
}

//pub type Result<T> = std::result::Result<T, WhitespaceError>;


