/*
    Copyright (C) 2021 Joshua Wade

    This file is part of Anthem.

    Anthem is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Anthem is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
    General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Anthem. If not, see <https://www.gnu.org/licenses/>.
*/

// cspell:ignore appender

use log::{self, debug, LevelFilter};
use log4rs::{
    self,
    append::file::FileAppender,
    config::{Appender, Root},
};

fn main() {
    let ipc_id = std::env::args().nth(1);

    if ipc_id.is_none() {
        return;
    }

    // TODO: This will still create a new file for each project worked on, as
    // well as one for each time the application is opened or a new project is
    // created but not saved. There maybe should at least be some cleanup done
    // on old logs?
    let appender = FileAppender::builder()
        .build(format!("logs/engine-{}.log", ipc_id.unwrap()))
        .expect("Could not build file log appender");

    let _handle = log4rs::init_config(
        log4rs::Config::builder()
            .appender(Appender::builder().build("logger", Box::new(appender)))
            .build(Root::builder().appender("logger").build(LevelFilter::Debug))
            // Probably should handle this more gracefully
            .expect("Could not build logger"),
    )
    .expect("Could not build logger");

    debug!("hi from log2rs");
}
