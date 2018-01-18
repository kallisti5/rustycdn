/*
 * Copyright 2016-2017, Haiku, Inc. All rights reserved.
 * Released under the terms of the MIT license.
 *
 * Authors:
 *    Alexander von Gluck IV <kallisti5@unixzen.com>
 *
 */

extern crate serde_json;

use iron::prelude::*;
use iron::status;
use router::Router;

use std::io::Read;
use std::path::{PathBuf};
use std::error::Error;

use repository::Repository;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AboutServer {
    pub api_version: i32,
    pub consumers: i32,
}

pub fn about(_: &mut Request, repo: Repository) -> Response {
    let about_data = AboutServer { api_version: 0, consumers: 0 };
	Response::with((status::Ok, serde_json::to_string(&about_data).unwrap()))
}
