use std::ffi::*;
use std::mem;
use windows::{
    Win32::{
        Security::SECURITY_ATTRIBUTES,
        System::Threading::{
            CREATE_BREAKAWAY_FROM_JOB, CREATE_NEW_PROCESS_GROUP, DETACHED_PROCESS,
            PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, STARTUPINFOW,
        },
        UI::WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW},
    },
    core::{BOOL, PCSTR, PCWSTR, PWSTR, Param, Result},
};
use windows_wrapper::formatw;

pub const RUMIA_DLL: &str = "rumia.dll";

pub fn launch_pvz(path: String) -> Option<PROCESS_INFORMATION> {
    unsafe {
        let mut si = STARTUPINFOW::default();
        si.cb = size_of_val(&si) as u32;
        let mut pi = PROCESS_INFORMATION::default();

        let mut commandu16 = formatw!("{}", path);
        let command_ptr = PWSTR(commandu16.as_mut_ptr());

        let dir = formatw!(
            "{}",
            std::env::current_dir()
                .unwrap_or_else(|_| ".".into())
                .to_string_lossy()
                .to_string()
        );

        let work_dir_ptr = PCWSTR(dir.as_ptr() as _);

        let dlls_ptr: Vec<PCSTR> = vec![PCSTR(RUMIA_DLL.as_ptr())];

        let result = DetourCreateProcessWithDllsW(
            PCWSTR::null(),
            Some(command_ptr),
            None,
            None,
            false,
            DETACHED_PROCESS | CREATE_BREAKAWAY_FROM_JOB | CREATE_NEW_PROCESS_GROUP,
            None,
            work_dir_ptr,
            &mut si,
            &mut pi,
            dlls_ptr.len() as u32,
            dlls_ptr.as_ptr(),
            0 as _,
        );

        if let Err(e) = result {
            let text = formatw!("{}({})", e.message(), e.code());
            MessageBoxW(
                None,
                PCWSTR(text.as_ptr()),
                PCWSTR::null(),
                MB_ICONERROR | MB_OK,
            );

            None
        } else {
            Some(pi)
        }
    }
}

#[inline]
#[allow(non_snake_case)]
pub unsafe fn DetourCreateProcessWithDllsW<P0, P7>(
    lpApplicationName: P0,
    lpCommandLine: Option<PWSTR>,
    lpProcessAttributes: Option<*const SECURITY_ATTRIBUTES>,
    lpThreadAttributes: Option<*const SECURITY_ATTRIBUTES>,
    bInheritHandles: bool,
    dwCreationFlags: PROCESS_CREATION_FLAGS,
    lpEnvironment: Option<*const core::ffi::c_void>,
    lpCurrentDirectory: P7,
    lpStartupInfo: *const STARTUPINFOW,
    lpProcessInformation: *mut PROCESS_INFORMATION,
    nDlls: u32,
    rlpDlls: *const PCSTR,
    pfCreateProcessW: *const c_void,
) -> Result<()>
where
    P0: Param<PCWSTR>,
    P7: Param<PCWSTR>,
{
    unsafe extern "system" {
        unsafe fn DetourCreateProcessWithDllsW(
            lpApplicationName: PCWSTR,
            lpCommandLine: PWSTR,
            lpProcessAttributes: *const SECURITY_ATTRIBUTES,
            lpThreadAttributes: *const SECURITY_ATTRIBUTES,
            bInheritHandles: BOOL,
            dwCreationFlags: PROCESS_CREATION_FLAGS,
            lpEnvironment: *const c_void,
            lpCurrentDirectory: PCWSTR,
            lpStartupInfo: *const STARTUPINFOW,
            lpProcessInformation: *mut PROCESS_INFORMATION,
            nDlls: u32,
            rlpDlls: *const PCSTR,
            pfCreateProcessW: *const c_void,
        ) -> BOOL;
    }

    unsafe {
        DetourCreateProcessWithDllsW(
            lpApplicationName.param().abi(),
            lpCommandLine.unwrap_or(mem::zeroed()) as _,
            lpProcessAttributes.unwrap_or(mem::zeroed()) as _,
            lpThreadAttributes.unwrap_or(mem::zeroed()) as _,
            bInheritHandles.into(),
            dwCreationFlags,
            lpEnvironment.unwrap_or(mem::zeroed()) as _,
            lpCurrentDirectory.param().abi(),
            lpStartupInfo,
            lpProcessInformation as _,
            nDlls,
            rlpDlls,
            pfCreateProcessW,
        )
        .ok()
    }
}
