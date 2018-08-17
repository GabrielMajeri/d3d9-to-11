macro_rules! impl_state {
    {
        $(#[$attr:meta])*
        pub struct $sname:ident {
            $($rs_name:ident : $rs_enum:path = $rs_default:expr),*;
            $($ss_name:ident : $ss_enum:path = $ss_default:expr),*;
            $($ts_name:ident : $ts_enum:path = $ts_default:expr),*;
            $($var:ident: $ty:ty,)*
        }
    } => {
        $(#[$attr])*
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $sname {
            $($rs_name: u32,)*
            $($ss_name: u32,)*
            $($ts_name: u32,)*
            $(pub(super) $var: Option<$ty>,)*
        }

        impl $sname {
            pub fn set_render_state(&mut self, state: D3DRENDERSTATETYPE, value: u32) {
                match state {
                    $($rs_enum => {
                        self.$rs_name = value;
                    },)*
                    _ => (),
                }
            }

            pub fn get_render_state(&self, state: D3DRENDERSTATETYPE) -> Option<u32> {
                match state {
                    $($rs_enum => Some(self.$rs_name),)*
                    _ => None,
                }
            }
        }

        impl Default for $sname {
            fn default() -> Self {
                Self {
                    $($rs_name: $rs_default,)*
                    $($ss_name: $ss_default,)*
                    $($ts_name: $ts_default,)*
                    $($var: None,)*
                }
            }
        }
    };
}
