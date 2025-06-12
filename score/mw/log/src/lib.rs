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
use log::{Log, Metadata, Record};

 struct MwLogger;

 impl Log for MwLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            // Enable all log levels
            true
        }
    
        fn log(&self, record: &Record) {
            let message = record.args().to_string();
            match record.level() {
                Level::Error => unsafe { mw_log_error(message.as_ptr() as *const _) },
                Level::Warn => unsafe { mw_log_warning(message.as_ptr() as *const _) },
                Level::Info => unsafe { mw_log_info(message.as_ptr() as *const _) },
                Level::Fatal => unsafe { mw_log_fatal(message.as_ptr() as *const _) },
                Level::Debug => unsafe { mw_log_debug(message.as_ptr() as *const _) },
                Level::Trace => unsafe { mw_log_trace(message.as_ptr() as *const _) },
            }
        }
    
        fn flush(&self) {}
 }