/********************************************************************************
 * Copyright (c) 2025 Contributors to the Eclipse Foundation
 *
 * See the NOTICE file(s) distributed with this work for additional
 * information regarding copyright ownership.
 *
 * This program and the accompanying materials are made available under the
 * terms of the Apache License Version 2.0 which is available at
 * https://www.apache.org/licenses/LICENSE-2.0
 *
 * SPDX-License-Identifier: Apache-2.0
 ********************************************************************************/
use cc::Build;

 fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut base = Build::new();
    base.cpp(true)
        .warnings(true)
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-Werror");

    base.include("./include")
        .file("src/assert.cpp")
        .file("src/charconv.cpp")
        .file("src/hash.cpp")
        .file("src/intrusive_forward_list.cpp")
        .file("src/jthread.cpp")
        .file("src/memory_resource.cpp")
        .file("src/stop_token.cpp")
        .file("src/string.cpp")
        .compile("futurecpp");
 }
