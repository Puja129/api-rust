/*
 * Copyright (c) 2016 General Electric Company. All rights reserved.
 *
 * The copyright to the computer software herein is the property of
 * General Electric Company. The software may be used and/or copied only
 * with the written permission of General Electric Company or in accordance
 * with the terms and conditions stipulated in the agreement/contract
 * under which the software has been supplied.
 *
 * author: apolo.yasuda@ge.com
 *
 */

use error_chain::error_chain;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

//extern {
// fn

//}

fn main() {
    download(
        "https://raw.githubusercontent.com/LIBRA-Release/libra/v0.1.0-reiwa/lib/arm/crypto.so",
        "crypto.so",
    )
    .ok();
    //download(
    //    "https://raw.githubusercontent.com/LIBRA-Release/libra/v0.1.0-reiwa/lib/arm/crypto.h",
    //    "crypto.h",
    //)
    //.ok();
}
#[tokio::main]
async fn download(target: &str, fname: &str) -> Result<()> {
    //let target =
    //    "https://raw.githubusercontent.com/LIBRA-Release/libra/v0.1.0-reiwa/lib/arm/crypto.h";
    let response = reqwest::get(target).await?;

    let path = Path::new(fname);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    //let content = response.bytes().await?;
    //file.write_all(content.as_bytes())?;
    let content = response.text().await?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
