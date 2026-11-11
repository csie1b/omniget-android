//! Abstraction over system trash with fallback for mobile/Android

use std::path::Path;

pub fn delete<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        trash::delete(path).map_err(|e| anyhow::anyhow!("{}", e))
    }
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        let p = path.as_ref();
        if p.is_dir() {
            std::fs::remove_dir_all(p).map_err(|e| anyhow::anyhow!("{}", e))
        } else {
            std::fs::remove_file(p).map_err(|e| anyhow::anyhow!("{}", e))
        }
    }
}
