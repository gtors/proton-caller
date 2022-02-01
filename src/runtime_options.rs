use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use crate::{error::Kind, throw};

/// Runtime options define at https://github.com/ValveSoftware/Proton/tree/proton_6.3-rc#runtime-config-options
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RuntimeOption {
    /// Convenience method for dumping a useful debug log
    log, // PROTON_LOG
    /// Use OpenGL-based wined3d instead of Vulkan-based DXVK for d3d11, d3d10, and d3d9.
    wined3d, // PROTON_USE_WINED3D
    /// Disable `d3d11.dll`, for d3d11 games which can fall back to and run better with d3d9.
    nod3d11, // PROTON_NO_D3D11
    /// Disable `d3d10.dll` and `dxgi.dll`, for d3d10 games which can fall back to and run better with d3d9.
    nod3d10, // PROTON_NO_D3D10
    /// Do not use eventfd-based in-process synchronization primitives.
    noesync, // PROTON_NO_ESYNC
    /// Do not use futex-based in-process synchronization primitives. (Automatically disabled on systems with no `FUTEX_WAIT_MULTIPLE` support.)
    nofsync, // PROTON_NO_FSYNC
    /// Enable NVIDIA's NVAPI GPU support library.
    enablenvapi, // PROTON_ENABLE_NVAPI
}

impl Display for RuntimeOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let opt = match self {
            RuntimeOption::log => "PROTON_LOG",
            RuntimeOption::wined3d => "PROTON_USE_WINED3D",
            RuntimeOption::nod3d11 => "PROTON_NO_D3D11",
            RuntimeOption::nod3d10 => "PROTON_NO_D3D10",
            RuntimeOption::noesync => "PROTON_NO_ESYNC",
            RuntimeOption::nofsync => "PROTON_NO_FSYNC",
            RuntimeOption::enablenvapi => "PROTON_ENABLE_NVAPI",
        };

        write!(f, "{}", opt)
    }
}

impl FromStr for RuntimeOption {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "log" => Ok(Self::log),
            "wined3d" => Ok(Self::wined3d),
            "nod3d11" => Ok(Self::nod3d11),
            "nod3d10" => Ok(Self::nod3d10),
            "noesync" => Ok(Self::noesync),
            "nofsync" => Ok(Self::nofsync),
            "enablenvapi" | "nvapi" => Ok(Self::enablenvapi),
            _ => throw!(Kind::ParseRuntimeOpt, "{} is not a runtime option", s),
        }
    }
}
