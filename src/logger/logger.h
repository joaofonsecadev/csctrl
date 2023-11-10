#pragma once

#include "spdlog/logger.h"

enum LoggerLevel
{
    Trace = 0,
    Debug,
    Info,
    Warn,
    Error,
    Critical
};

class Logger
{
public:
    static void Init();
    static std::shared_ptr<spdlog::logger> GetStdoutLogger() { return m_StdoutLogger; }
    static spdlog::level::level_enum LoggerLevelToSpdlogLevel(const LoggerLevel& LogLevel);
private:
    static std::shared_ptr<spdlog::logger> m_StdoutLogger;
};

