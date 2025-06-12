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
#include "mw/log/logging.h"

extern "C" {
    void mw_log_info(const char* message) {
        mw::LogInfo() << message;
    }

    void mw_log_warning(const char* message) {
        mw::LogWarning() << message;
    }

    void mw_log_error(const char* message) {
        mw::LogError() << message;
    }

    void mw_log_debug(const char* message) {
        mw::LogDebug() << message;
    }

    void mw_log_fatal(const char* message) {
        mw::LogFatal() << message;
    }

    void mw_log_trace(const char* message) {
        mw::LogTrace() << message;
    }
   
}
