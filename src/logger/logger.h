#pragma once

#include "spdlog/logger.h"

class Logger
{
public:
    static void Init();
    static std::shared_ptr<spdlog::logger> GetStdoutLogger() { return m_StdoutLogger; }

private:
    static std::shared_ptr<spdlog::logger> m_StdoutLogger;
};

