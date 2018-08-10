#pragma once

/// Logging functions which use variadic templates.
namespace log {
    template <typename ...T>
    inline void error(T&&... params) {
        std::cerr << "error:\t";
        (std::cerr << ... << std::forward<T>(params)) << '\n';
    }

    template <typename ...T>
    inline void warn(T&&... params) {
        std::cerr << "warn:\t";
        (std::cerr << ... << std::forward<T>(params)) << '\n';
    }

    template <typename ...T>
    inline void info(T&&... params) {
        std::cerr << "info:\t";
        (std::cerr << ... << std::forward<T>(params)) << '\n';
    }
}

std::ostream& operator<<(std::ostream& os, REFIID guid);
