//
// Copyright 2024, DornerWorks
//
// SPDX-License-Identifier: BSD-2-Clause
//

#![no_std]
#![recursion_limit = "256"]

#[cfg(feature = "ethernet")]
pub mod gem;
#[cfg(feature = "uart")]
pub mod uart;

#[cfg(test)]
mod tests {
    use super::*;
}
