use anyhow::Result;
use windows::Win32::System::Console::AllocConsole;

pub fn alloc_console() -> Result<()> {
    unsafe {
        AllocConsole()?;

        Ok(())
    }
}
