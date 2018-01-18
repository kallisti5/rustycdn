/*
 * Copyright 2016-2017, Haiku, Inc. All rights reserved.
 * Released under the terms of the MIT license.
 *
 * Authors:
 *    Alexander von Gluck IV <kallisti5@unixzen.com>
 *
 */

extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path,PathBuf};
use std::ffi::OsStr;

use iron::prelude::*;
use iron::status;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub path: Option<PathBuf>,
    pub name: String,
    pub base: String,
    pub rw: bool
}

pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Repository, Box<Error>> {
    let info_path = path.as_ref().join(".info.json");
    let file = File::open(info_path)?;
    let u = serde_json::from_reader(file)?;
    Ok(u)
}

impl Repository {
    /// Create a new empty repository
    pub fn new() -> Repository {
        Repository {
            path: None,
            name: "".to_string(),
            base: "/".to_string(),
            rw: false
        }
    }
    /// List artifacts available in repo
    pub fn artifacts(&self) -> Vec<PathBuf> {
        let mut artifacts: Vec<PathBuf> = Vec::new();
        let repo = self.clone();
        let repo_path = repo.path.unwrap().clone();
        let objects = match fs::read_dir(&repo_path) {
            Ok(p) => (p),
            Err(e) => {
                println!("[e] Error locating artifacts at {}: {}", repo_path.display(), e.description());
                return artifacts;
            }
        };
        for object in objects {
            let object_path = object.unwrap().path();
            if object_path.file_name() == Some(OsStr::new(".info.json")) {
                continue;
            }
            artifacts.push(object_path);
        }
        return artifacts;
    }
    pub fn get(_: &mut Request, repo: Repository) -> Response {
        Response::with((status::Ok, serde_json::to_string(&repo.artifacts()).unwrap()))
    }
    pub fn put(_: &mut Request, repo: Repository) -> Response {
        Response::with((status::Ok, serde_json::to_string(&repo).unwrap()))
    }
}
