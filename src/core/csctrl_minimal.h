#pragma once

#include <cstdint>
#include "tracy/Tracy.hpp"

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
#define SPDLOG_ACTIVE_LEVEL SPDLOG_LEVEL_TRACE
#endif

#include <spdlog/spdlog.h>
#include "logger/logger.h"
#define CSCTRL_LOG(LogLevel, ...) \
    SPDLOG_LOGGER_CALL(Logger::GetStdoutLogger(), Logger::LoggerLevelToSpdlogLevel(LogLevel), __VA_ARGS__); \
    if (LogLevel == LoggerLevel::Critical) \
        assert(false && __VA_ARGS__)
