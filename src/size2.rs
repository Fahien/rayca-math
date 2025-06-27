// Copyright © 2023-2025
// Author: Antonio Caggiano <info@antoniocaggiano.eu>
// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Size2 {
    pub width: u32,
    pub height: u32,
}

impl Size2 {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl std::fmt::Display for Size2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}x{}", self.width, self.height))
    }
}
