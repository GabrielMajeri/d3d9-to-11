#include "log.hpp"

#include <iomanip>

std::ostream& operator<<(std::ostream& os, REFIID guid) {
    os << std::hex << std::setfill('0');

    os << std::setw(8) << guid.Data1 << '-';

    os << std::setw(4) << guid.Data2 << '-';

    os << std::setw(4) << guid.Data3 << '-';

    os << std::setw(2);
    os << static_cast<short>(guid.Data4[0]) << static_cast<short>(guid.Data4[1])
        << '-'
        << static_cast<short>(guid.Data4[2]) << static_cast<short>(guid.Data4[3])
        << static_cast<short>(guid.Data4[4]) << static_cast<short>(guid.Data4[5])
        << static_cast<short>(guid.Data4[6]) << static_cast<short>(guid.Data4[7]);

    return os;
}
