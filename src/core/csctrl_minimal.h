#pragma once

#include <cstdint>
#include <spdlog/spdlog.h>
#include "logger/logger.h"

typedef int8_t      int8;
typedef int16_t     int16;
typedef int32_t     int32;
typedef int64_t     int64;
typedef uint8_t     uint8;
typedef uint16_t    uint16;
typedef uint32_t    uint32;
typedef uint64_t    uint64;

#ifdef NDEBUG
#define CSCTRL_SHIPPING 1
#else
#define CSCTRL_SHIPPING 0
#endif

#define CSCTRL_LOG(LogLevel, ...) \
    SPDLOG_LOGGER_CALL(Logger::GetStdoutLogger(), LogLevel, __VA_ARGS__); \
    if (LogLevel == spdlog::level::critical) \
        assert(false && __VA_ARGS__)
