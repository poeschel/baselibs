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
use log::{Level, Log, Metadata, Record};

use std::ffi::c_char;

unsafe extern "C" {
     fn mw_log_info(message: *const c_char);
     fn mw_log_warn(message: *const c_char);
     fn mw_log_error(message: *const c_char);
     fn mw_log_debug(message: *const c_char);
    // fn mw_log_fatal(message: *const c_char);
     fn mw_log_verbose(message: *const c_char);
 }

#[repr(C)]
pub struct MwLogger;

impl Log for MwLogger {
        fn enabled(&self, _metadata: &Metadata) -> bool {
            // Enable all log levels
            true
        }
    
        fn log(&self, record: &Record) {
            let message = record.args().to_string();
            match record.level() {
                Level::Error => unsafe { mw_log_error(message.as_ptr() as *const _) },
                Level::Warn => unsafe { mw_log_warn(message.as_ptr() as *const _) },
                Level::Info => unsafe { mw_log_info(message.as_ptr() as *const _) },
                // Level::Fatal => unsafe { mw_log_fatal(message.as_ptr() as *const _) },
                Level::Debug => unsafe { mw_log_debug(message.as_ptr() as *const _) },
                Level::Trace => unsafe { mw_log_verbose(message.as_ptr() as *const _) },
            }
        }
    
        fn flush(&self) {}
 }
