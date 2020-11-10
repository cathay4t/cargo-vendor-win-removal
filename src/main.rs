// SPDX-License-Identifier: Apache-2.0

//
// Steps:
//  1. Amend `Cargo.toml` by removing all `[target."cfg(windows)".dependencies]`
//     sections.
//  2. Amend `.cargo-checksum.json` with empty `files`.
//  3. Remove windows platform dependencies from vendor folder.

fn main() {
    println!("Hello, world!");
}
