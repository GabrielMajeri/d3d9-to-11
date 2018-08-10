#pragma once

/// Smart pointer wrapping a COM interface.
///
/// Inspired by https://msdn.microsoft.com/en-us/magazine/dn904668.aspx
template <typename I>
class ComPtr final {
public:
    // This private class is used to hide away the IUnknown methods from the wrapped type.
    // This way, only this class is allowed to handle reference counting.
    class HideIUnknown: public I {
        HideIUnknown() = delete;
        ~HideIUnknown() = delete;

        HRESULT WINAPI QueryInterface(REFIID riid, void** ppvObject);
        ULONG WINAPI AddRef();
        ULONG WINAPI Release();
    };

    /// Default constructor.
    ComPtr() noexcept = default;

    /// Copy constructor. Templated to allow covariance.
    template <typename T>
    ComPtr(const ComPtr<T>& rhs) noexcept
        : ptr { rhs.ptr } {
        add_ref();
    }

    /// Move constructor.
    template <typename T>
    ComPtr(ComPtr<T>&& rhs) noexcept
        : ptr { std::exchange(rhs.ptr, nullptr) } {
    }

    /// Default destructor.
    ~ComPtr() noexcept {
        *this = nullptr;
    }

    template <typename T>
    ComPtr& operator=(const ComPtr<T>& rhs) noexcept {
        if (ptr != rhs.ptr) {
            *this = nullptr;
            ptr = rhs.ptr;
            add_ref();
        }

        return *this;
    }

    template <typename T>
    ComPtr& operator=(ComPtr<T>&& rhs) noexcept {
        if (ptr != rhs.ptr) {
            *this = nullptr;
            ptr = std::exchange(rhs.ptr, nullptr);
        }

        return *this;
    }

    /// To reset a ComPtr, assign nullptr to it.
    ComPtr& operator=(std::nullptr_t) noexcept {
        auto tmp = ptr;
        if (tmp) {
            ptr = nullptr;
            tmp->Release();
        }
        return *this;
    }

    explicit operator bool() const noexcept {
        return ptr != nullptr;
    }

    /// Returns the pointer stored in this interface,
    /// without changing the reference count.
    I* as_raw() const noexcept {
        return ptr;
    }

    /// Disowns this pointer and returns the wrapped value.
    I* into_raw() noexcept {
        auto tmp = ptr;
        *this = nullptr;
        return tmp;
    }

    HideIUnknown* operator->() const noexcept {
        return static_cast<HideIUnknown*>(ptr);
    }

    I** operator&() noexcept {
        assert(!ptr);
        return &ptr;
    }

    /// Simplifies obtaining the UUID of the wrapped interface.
    inline const auto& uuid() const noexcept {
        return __uuidof(I);
    }

private:
    void add_ref() const noexcept {
        if (ptr) {
            ptr->AddRef();
        }
    }

    I* ptr = nullptr;

    template <typename T>
    friend class ComPtr;

    static_assert(std::is_base_of_v<IUnknown, I>, "ComPtr can only manage COM interfaces");
};

template <typename I>
inline void swap(ComPtr<I>& lhs, ComPtr<I>& rhs) noexcept {
    std::swap(lhs.ptr, rhs.ptr);
}

template <typename I>
I* ref(I* iface) {
    assert(iface && "ref received a null pointer");
    iface->AddRef();
    return iface;
}
