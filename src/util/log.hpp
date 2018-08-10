#pragma once

/// Logging functions which use variadic templates.
namespace log {
    template <typename ...T>
    inline void error(T&&... params) {
        (std::cerr << ... << std::forward<T>(params)) << '\n';
    }
}

std::ostream& operator<<(std::ostream& os, REFIID guid);
