#pragma once

#include <string>
#include <string_view>
#include <sstream>

namespace str {
    std::string convert(std::wstring_view ws);
    std::wstring convert(std::string_view s);

    template <typename ...Ts>
    std::string join(Ts&&... ts) {
        std::stringstream ss;
        (ss << ... << std::forward<Ts>(ts));
        return ss.str();
    }
}
