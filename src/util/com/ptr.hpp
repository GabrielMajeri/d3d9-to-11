#pragma once

template <typename I>
I* ref(I* iface) {
    assert(iface && "ref received a null pointer");
    iface->AddRef();
    return iface;
}
