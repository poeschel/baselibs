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
use std::fs;
use std::path::Path;

 fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=logging_identifier");

    Build::new()
        .cpp(true)
        .include("../..")
        .include("../../..")
        .include("../../static_reflection_with_serialization/visitor/include")
        .include("../../language/futurecpp/include")
        .file("src/mw_log_ffi.cpp")
        .compile("mw_log_ffi");

    Build::new()
        .cpp(true)
        .include("../../..")
        .include("../../static_reflection_with_serialization/visitor/include")
        .include("../../language/futurecpp/include")
        .file("log_stream.cpp")
        .file("log_stream_error.cpp")
        .file("log_stream.h")
        .compile("log_stream");

    Build::new()
        .cpp(true)
        .include("../../..")
        .include("../../language/futurecpp/include")
        .file("recorder.cpp")
        .file("recorder.h")
        .compile("recorder");

    Build::new()
        .cpp(true)
        .include("../../..")
        .include("../../language/futurecpp/include")
        .file("log_common.cpp")
        .file("log_level.cpp")
        .file("slot_handle.cpp")
        .file("log_common.h")
        .file("log_level.h")
        .file("log_types.h")
        .file("slot_handle.h")
        .compile("shared_types");

    Build::new()
        .cpp(true)
        .include("../../..")
        .include("../../static_reflection_with_serialization/visitor/include")
        .include("../../language/futurecpp/include")
        .file("log_stream_factory.cpp")
        .file("logger.cpp")
        .file("logger_container.cpp")
        .file("logging.cpp")
        .file("runtime.cpp")
        .file("log_stream_factory.h")
        .file("logger.h")
        .file("logger_container.h")
        .file("logging.h")
    .file("runtime.h")
    .compile("frontend");

    Build::new()
        .cpp(true)
        .include("../../..")
        .include("../../language/futurecpp/include")
        .file("irecorder_factory.cpp")
        .file("irecorder_factory.h")
        .compile("recorder_interface");


 }