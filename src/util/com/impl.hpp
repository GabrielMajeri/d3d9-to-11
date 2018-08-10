#pragma once

/// This mix-in should be inherited by classes which need to implement a COM interface.
///
/// It manages reference counting and interface querying for them.
///
/// It is very important that all the base interfaces, excluding IUnknown,
/// get listed in the template arguments. Otherwise, QueryInterface might not work.
template <typename ...Base>
class ComImpl: public virtual Base... {
public:
    virtual ~ComImpl() = default;

    HRESULT WINAPI QueryInterface(REFIID riid, void** ppvObject) override {
        if (ppvObject == nullptr)
            return E_INVALIDARG;

        if (riid == __uuidof(IUnknown)
            // Use a C++17 fold expression to check all base interfaces.
            || (... || (riid == __uuidof(Base)))) {
            *ppvObject = ref(this);
            return S_OK;
        } else {
            *ppvObject = nullptr;
            log::error("Unknown interface query: ", riid);
            return E_NOTIMPL;
        }
    }

    ULONG WINAPI AddRef() override {
        return ++refs;
    }

    ULONG WINAPI Release() override {
        const auto count = --refs;
        if (count == 0) {
            delete this;
        }
        return count;
    }

private:
    std::atomic_uint refs{};
};
