// Copyright 2015-2017 Benjamin Fry <benjaminfry@me.com>
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// https://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! line info

use std::string::String;

/// TODO: add documents
#[derive(
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Hash,
    Clone,
    serde::Serialize,
    serde::Deserialize,
)]
pub struct LineInfo {
    /// country info, optional
    pub country: Option<String>,
    /// province info, optional
    pub province: Option<String>,
    /// province info, optional
    pub city: Option<String>,
    /// province info, optional
    pub isp: Option<String>,
}
