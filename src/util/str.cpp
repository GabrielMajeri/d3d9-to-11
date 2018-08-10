#include "str.hpp"

namespace str {
    std::string convert(std::wstring_view ws) {
        const auto size = WideCharToMultiByte(CP_UTF8, 0,
            ws.data(), -1,
            nullptr, 0,
            nullptr, nullptr);

        std::string str(size, 0);

        WideCharToMultiByte(CP_UTF8, 0,
            ws.data(), ws.size(),
            str.data(), str.size(),
            nullptr, nullptr);

        return str;
    }

    std::wstring convert(std::string_view s) {
        const auto size = MultiByteToWideChar(CP_UTF8, 0,
            s.data(), -1,
            nullptr, 0);

        std::wstring wstr(size, 0);

        MultiByteToWideChar(CP_UTF8, 0,
            s.data(), s.size(),
            wstr.data(), wstr.size());

        return wstr;
    }
}
