/*
 * Copyright 2016-2018, Haiku, Inc. All rights reserved.
 * Released under the terms of the MIT license.
 *
 * Authors:
 *	Alexander von Gluck IV <kallisti5@unixzen.com>
 *
 */
extern crate iron;
extern crate router;
extern crate getopts;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::env;
use std::process;
use std::fs;
use std::path::{Path,PathBuf};
use std::error::Error;

use getopts::Options;

use iron::prelude::*;
use iron::Handler;
use router::Router;

use repository::Repository;

mod repository;
mod control_routes;

struct RouteHandler {
    repo: Repository,
	processor: fn(&mut Request, Repository) -> Response
}

impl Handler for RouteHandler {
	fn handle(&self, req: &mut Request) -> IronResult<Response> {
		Ok((self.processor)(req, self.repo.clone()))
	}
}

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} [options] <prefix>", program);
	print!("{}", opts.usage(&brief));
}

fn main() {
	let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
	let mut opts = Options::new();
	opts.optopt("l", "listener", "set listener address", "127.0.0.1");
	opts.optopt("p", "port", "set listener port", "8889");
	opts.optflag("h", "help", "print this help");

	let matches = match opts.parse(&args[1..]) {
		Ok(m) => { m },
		Err(f) => {
			println!("Error: {}", f.to_string());
			return;
		}
	};

	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}

	let prefix = if !matches.free.is_empty() {
		Path::new(&matches.free[0])
	} else {
		print_usage(&program, opts);
		return;
	};

	if !prefix.exists() {
		println!("Error: {} doesn't exist!", prefix.display());
		process::exit(1);
	}

	let address = match matches.opt_str("l") {
		Some(x) => x,
		None => "127.0.0.1".to_string(),
	};
	let port = match matches.opt_str("p") {
		Some(x) => x,
		None => "8889".to_string(),
	};

	let listener = format!("{}:{}", address, port);
	println!("Startup: Using {} for repositories", prefix.display());
	println!("Startup: Listening on {}", listener);

    let mut repositories: Vec<Repository> = Vec::new();

    // Collect repositories
    for object in fs::read_dir(prefix).unwrap() {
        let obj_path = match object {
            Err(why) => {
                println!("[e] Error accessing {}: {}", prefix.display(), why.description());
                continue;
            },
            Ok(f) => f.path()
        };
        let mut repository = match repository::from_path(&obj_path) {
            Err(why) => {
                println!("[e] Invalid repository at {}: {}", obj_path.display(), why.description());
                continue;
            },
            Ok(r) => r,
        };
        repository.path = Some(obj_path.clone());
        println!("Name: {:?}", repository);
        repositories.push(repository);
    }

	// Attach routes
	{
		let mut router = Router::new();

        // Control
        router.get("/about",
            RouteHandler { repo: Repository::new(), processor: control_routes::about }, "about");

        // Attach Repository GET Routes
        for repo in repositories {
            let base = repo.base.clone();
            router.get(&base,
                RouteHandler { repo: repo.clone(), processor: Repository::get }, repo.name.clone());
            if repo.rw {
                router.put(&base,
                    RouteHandler { repo: repo.clone(), processor: Repository::put }, repo.name.clone());
            }
        }

		Iron::new(router).http(listener).unwrap();
	}
}
