// SPDX-License-Identifier: Apache-2.0

//
// Steps:
//  1. Amend `Cargo.toml` by removing all `[target."cfg(windows)".dependencies]`
//     sections and note down the removed crates.
//  2. Amend `Cargo.toml` for features depending on windows dependencies.
//  3. Amend `.cargo-checksum.json` with empty `files`.
//  4. Remove windows platform dependencies from vendor folder.

fn main() {
    println!("Hello, world!");
}
