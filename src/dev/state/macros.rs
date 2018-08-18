macro_rules! impl_state {
    {
        $(#[$attr:meta])*
        pub struct $sname:ident {
            $($rs_name:ident : $rs_enum:path = $rs_default:expr),*;
            MAX_SAMPLERS = $maxn:expr;
            $($ss_name:ident : $ss_enum:path = $ss_default:expr),*;
            $($ts_name:ident : $ts_enum:path = $ts_default:expr),*;
            $($var:ident: $ty:ty = $var_def:expr,)*
        }
    } => {
        $(#[$attr])*
        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct $sname {
            $(pub $rs_name: u32,)*
            pub ss: [SamplerState; $maxn],
            pub ts: [TextureState; $maxn],
            $(pub $var: $ty,)*
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct SamplerState {
            $(pub $ss_name: u32,)*
        }

        #[derive(Debug, Copy, Clone, Eq, PartialEq)]
        pub struct TextureState {
            $(pub $ts_name: u32,)*
        }

        // In some cases the state might consist of only one variable,
        // even then it's simpler to use a match statement everywhere.
        #[allow(single_match)]
        impl $sname {
            /// Sets a render state variable.
            pub fn set_render_state(&mut self, state: D3DRENDERSTATETYPE, value: u32) {
                match state {
                    $($rs_enum => self.$rs_name = value,)*
                    _ => (),
                }
            }

            /// Retrieves the value of a render state variable.
            pub fn get_render_state(&self, state: D3DRENDERSTATETYPE) -> Option<u32> {
                match state {
                    $($rs_enum => Some(self.$rs_name),)*
                    _ => None,
                }
            }

            /// Sets a sampler state variable.
            pub fn set_sampler_state(&mut self, sampler: u32, ty: D3DSAMPLERSTATETYPE, value: u32) {
                self.ss.get_mut(sampler as usize)
                    .map(|ss| match ty {
                        $($ss_enum => ss.$ss_name = value,)*
                        _ => (),
                    });
            }

            /// Retrieves the value of a sampler state variable.
            pub fn get_sampler_state(&self, sampler: u32, ty: D3DSAMPLERSTATETYPE) -> u32 {
                self.ss.get(sampler as usize)
                    .map(|ss| match ty {
                        $($ss_enum => ss.$ss_name,)*
                        _ => 0,
                    })
                    .unwrap_or_default()
            }

            /// Sets a texture stage state variable.
            pub fn set_texture_stage_state(&mut self, stage: u32, ty: D3DTEXTURESTAGESTATETYPE, value: u32) {
                self.ts.get_mut(stage as usize)
                    .map(|ts| match ty {
                        $($ts_enum => ts.$ts_name = value,)*
                        _ => (),
                    });
            }

            /// Retrieves the value of a texture stage state variable.
            pub fn get_texture_stage_state(&self, stage: u32, ty: D3DTEXTURESTAGESTATETYPE) -> u32 {
                self.ts.get(stage as usize)
                    .map(|ts| match ty {
                        $($ts_enum => ts.$ts_name,)*
                        _ => 0,
                    })
                    .unwrap_or_default()
            }
        }

        impl Default for $sname {
            fn default() -> Self {
                Self {
                    $($rs_name: $rs_default,)*
                    ss: [SamplerState::default(); $maxn],
                    ts: [TextureState::default(); $maxn],
                    $($var: $var_def,)*
                }
            }
        }

        impl Default for SamplerState {
            fn default() -> Self {
                Self {
                    $($ss_name: $ss_default,)*
                }
            }
        }

        impl Default for TextureState {
            fn default() -> Self {
                Self {
                    $($ts_name: $ts_default,)*
                }
            }
        }

    };
}
