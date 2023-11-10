#include "logger.h"
#include <memory>
#include "spdlog/sinks/stdout_color_sinks.h"

std::shared_ptr<spdlog::logger> Logger::m_StdoutLogger;

void Logger::Init()
{
    auto StdoutLogSink = std::make_shared<spdlog::sinks::stdout_color_sink_mt>();
    m_StdoutLogger = std::make_shared<spdlog::logger>("CSCTRL", StdoutLogSink);
    m_StdoutLogger->set_level(spdlog::level::trace);
    m_StdoutLogger->set_pattern("[%T.%e] %-37!! %^%5!l%$: %v");
}

spdlog::level::level_enum Logger::LoggerLevelToSpdlogLevel(const LoggerLevel& LogLevel)
{
    switch (LogLevel)
    {
    case Trace:
        return spdlog::level::trace;
    case Debug:
        return spdlog::level::debug;
    case Info:
        return spdlog::level::info;
    case Warn:
        return spdlog::level::warn;
    case Error:
        return spdlog::level::err;
    case Critical:
        return spdlog::level::critical;
    }
    return spdlog::level::info;
}
