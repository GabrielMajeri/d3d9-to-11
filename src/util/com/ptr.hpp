#pragma once

/// Smart pointer wrapping a COM interface.
///
/// Inspired by https://msdn.microsoft.com/en-us/magazine/dn904668.aspx
template <typename I>
class ComPtr final {
public:
    // This private class is used to hide away the IUnknown methods from the wrapped type.
    // This way, only this class is allowed to handle reference counting.
    class HideIUnknown final: public I {
        HRESULT WINAPI QueryInterface(REFIID riid, void** ppvObject);
        ULONG WINAPI AddRef();
        ULONG WINAPI Release();
    };

    /// Default constructor.
    ComPtr() noexcept = default;

    /// Copy constructor. Templated to allow covariance.
    template <typename T>
    ComPtr(const ComPtr<T>& rhs)
        : ptr { rhs } {
        add_ref();
    }

    /// Move constructor.
    template <typename T>
    ComPtr(const ComPtr<T>&& rhs)
        : ptr { std::exchange(rhs.ptr, nullptr) } {
    }

    /// Default destructor.
    ~ComPtr() noexcept {
        release();
    }

    template <typename T>
    ComPtr& operator=(const ComPtr<T>& rhs) noexcept {
        copy(rhs.ptr);
        return *this;
    }

    template <typename T>
    ComPtr& operator=(ComPtr<T>&& rhs) noexcept {
        move(rhs);
        return *this;
    }

    /// To reset a ComPtr, assign nullptr to it.
    ComPtr& operator=(std::nullptr_t) noexcept {
        release();
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
        return ptr;
    }

    I** operator&() noexcept {
        assert(!ptr);
        return &ptr;
    }

    /// Simplifies obtaining the UUID of the wrapped interface.
    inline const static auto& uuid = __uuidof(I);

private:
    void add_ref() const noexcept {
        if (ptr) {
            ptr->AddRef();
        }
    }

    void release() noexcept {
        I* tmp = ptr;
        if (tmp) {
            ptr = nullptr;
            tmp->Release();
        }
    }

    void copy(I* rhs) noexcept {
        if (ptr != rhs) {
            release();
            ptr = rhs;
            add_ref();
        }
    }

    template <typename T>
    void move(ComPtr<T>& rhs) noexcept {
        if (ptr != rhs.ptr) {
            release();
            ptr = std::exchange(rhs.ptr, nullptr);
        }
    }

    I* ptr = nullptr;

    template <typename T>
    friend class ComPtr;

    static_assert(std::is_base_of_v<IUnknown, I>, "ComPtr can only manage COM interfaces");
};

template <typename I>
I* ref(I* iface) {
    assert(iface && "ref received a null pointer");
    iface->AddRef();
    return iface;
}
