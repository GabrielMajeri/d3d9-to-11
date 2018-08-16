/// Macro which ensures a certain closure will only be run once.
///
/// Usually used to avoid logging the same warning multiple times.
macro_rules! run_once {
    ($closure: expr) => {{
        use std::sync::Once;
        static LOGGED: Once = Once::new();
        LOGGED.call_once($closure);
    }};
}

macro_rules! impl_iunknown {
    (struct $struct_name:ty : $($ifaces:ident),*) => {
        #[implementation(IUnknown)]
        impl $struct_name {
            fn query_interface(&mut self, riid: &winapi::shared::guiddef::GUID, obj: &mut usize) -> i32 {
                use winapi::Interface;
                use winapi::shared::{guiddef::IsEqualGUID, winerror::{S_OK, E_NOTIMPL}};

                *obj = 0;

                if $(IsEqualGUID(riid, &$ifaces::uuidof())) || * {
                    *obj = self as *mut _ as usize;
                    self.add_ref();
                    S_OK
                } else {
                    E_NOTIMPL
                }
            }

            fn add_ref(&mut self) -> u32 {
                let prev = self.refs.fetch_add(1, Ordering::SeqCst);
                prev + 1
            }

            fn release(&mut self) -> u32 {
                let prev = self.refs.fetch_sub(1, Ordering::SeqCst);
                if prev == 1 {
                    let _box = unsafe { Box::from_raw(self as *mut _) };
                }
                prev - 1
            }
        }
    };
}
