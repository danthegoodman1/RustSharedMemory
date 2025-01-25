use std::io;

#[cfg(target_os = "linux")]
mod platform {
    use libc::{
        c_void, ftruncate, mmap, shm_open, shm_unlink, MAP_FAILED, MAP_SHARED, O_CREAT, O_RDWR,
        PROT_READ, PROT_WRITE, S_IRUSR, S_IWUSR,
    };
    use std::ffi::CString;
    use std::io;
    use std::os::unix::io::RawFd;
    use std::ptr;
    pub type ShmId = RawFd;
    pub unsafe fn create(size: usize, id: &str) -> Result<(*mut u8, ShmId), io::Error> {
        let c_id = CString::new(id).unwrap();
        let shm_id = shm_open(c_id.as_ptr(), O_CREAT | O_RDWR, S_IRUSR | S_IWUSR);
        if shm_id == -1 {
            return Err(io::Error::last_os_error());
        }
        let res = ftruncate(shm_id, size as libc::off_t);
        if res == -1 {
            shm_unlink(c_id.as_ptr());
            return Err(io::Error::last_os_error());
        }
        let addr = mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            shm_id,
            0,
        );
        if addr == MAP_FAILED {
            shm_unlink(c_id.as_ptr());
            return Err(io::Error::last_os_error());
        }
        Ok((addr as *mut u8, shm_id))
    }
    pub unsafe fn open(size: usize, id: &str) -> Result<(*mut u8, ShmId), io::Error> {
        let c_id = CString::new(id).unwrap();
        let shm_id = shm_open(c_id.as_ptr(), O_RDWR, 0);
        if shm_id == -1 {
            return Err(io::Error::last_os_error());
        }
        let addr = mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            shm_id,
            0,
        );
        if addr == MAP_FAILED {
            shm_unlink(c_id.as_ptr());
            return Err(io::Error::last_os_error());
        }
        Ok((addr as *mut u8, shm_id))
    }
    pub unsafe fn unmap(addr: *mut u8, size: usize) -> Result<(), io::Error> {
        let res = libc::munmap(addr as *mut c_void, size);
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub unsafe fn close(id: ShmId) -> Result<(), io::Error> {
        let res = libc::close(id);
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub unsafe fn destroy(id: &str) -> Result<(), io::Error> {
        let c_id = CString::new(id).unwrap();
        let res = shm_unlink(c_id.as_ptr());
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}
#[cfg(target_os = "macos")]
mod platform {
    use std::ffi::CString;
    use std::io;
    use std::ptr;
    use libc::c_uint;
    use libc::{
        c_int, c_void, ftruncate, mmap, shm_open, shm_unlink, MAP_FAILED, MAP_SHARED, O_CREAT,
        O_RDWR, PROT_READ, PROT_WRITE, S_IRUSR, S_IWUSR,
    };
    use std::os::unix::io::RawFd;
    pub type ShmId = RawFd;
    pub unsafe fn create(size: usize, id: &str) -> Result<(*mut u8, ShmId), io::Error> {
        let c_id = CString::new(id).unwrap();
        let shm_id = shm_open(c_id.as_ptr(), O_CREAT | O_RDWR, S_IRUSR as c_uint | S_IWUSR as c_uint);
        if shm_id == -1 {
            return Err(io::Error::last_os_error());
        }
        let res = ftruncate(shm_id, size as libc::off_t);
        if res == -1 {
            shm_unlink(c_id.as_ptr());
            return Err(io::Error::last_os_error());
        }
        let addr = mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            shm_id,
            0,
        );
        if addr == MAP_FAILED {
            shm_unlink(c_id.as_ptr());
            return Err(io::Error::last_os_error());
        }
        Ok((addr as *mut u8, shm_id))
    }
    pub unsafe fn open(size: usize, id: &str) -> Result<(*mut u8, ShmId), io::Error> {
        let c_id = CString::new(id).unwrap();
        let shm_id = shm_open(c_id.as_ptr(), O_RDWR, 0);
        if shm_id == -1 {
            return Err(io::Error::last_os_error());
        }
        let addr = mmap(
            ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            shm_id,
            0,
        );
        if addr == MAP_FAILED {
            shm_unlink(c_id.as_ptr());
            return Err(io::Error::last_os_error());
        }
        Ok((addr as *mut u8, shm_id))
    }
    pub unsafe fn unmap(addr: *mut u8, size: usize) -> Result<(), io::Error> {
        let res = libc::munmap(addr as *mut c_void, size);
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub unsafe fn close(id: ShmId) -> Result<(), io::Error> {
        let res = libc::close(id);
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub unsafe fn destroy(id: &str) -> Result<(), io::Error> {
        let c_id = CString::new(id).unwrap();
        let res = shm_unlink(c_id.as_ptr());
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use libc::{
        c_void, CloseHandle, CreateFileMappingW, GetLastError, MapViewOfFile, OpenFileMappingW,
        UnmapViewOfFile, DWORD, FILE_MAP_ALL_ACCESS, HANDLE, INVALID_HANDLE_VALUE,
    };
    use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr::null_mut};

    pub type ShmId = HANDLE;

    fn to_utf16(s: &str) -> Vec<u16> {
        OsStr::new(s).encode_wide().chain(Some(0)).collect()
    }

    pub unsafe fn create(size: usize, id: &str) -> Result<(*mut u8, ShmId), io::Error> {
        let name_utf16 = to_utf16(id);
        let shm_id = CreateFileMappingW(
            INVALID_HANDLE_VALUE,
            ptr::null_mut(),
            FILE_MAP_ALL_ACCESS,
            0,
            size as DWORD,
            name_utf16.as_ptr(),
        );
        if shm_id == null_mut() {
            return Err(io::Error::last_os_error());
        }
        let addr = MapViewOfFile(shm_id, FILE_MAP_ALL_ACCESS, 0, 0, size);
        if addr == null_mut() {
            CloseHandle(shm_id);
            return Err(io::Error::last_os_error());
        }
        Ok((addr as *mut u8, shm_id))
    }
    pub unsafe fn open(size: usize, id: &str) -> Result<(*mut u8, ShmId), io::Error> {
        let name_utf16 = to_utf16(id);
        let shm_id = OpenFileMappingW(FILE_MAP_ALL_ACCESS, 0, name_utf16.as_ptr());
        if shm_id == null_mut() {
            return Err(io::Error::last_os_error());
        }
        let addr = MapViewOfFile(shm_id, FILE_MAP_ALL_ACCESS, 0, 0, size);
        if addr == null_mut() {
            CloseHandle(shm_id);
            return Err(io::Error::last_os_error());
        }
        Ok((addr as *mut u8, shm_id))
    }
    pub unsafe fn unmap(addr: *mut u8, _size: usize) -> Result<(), io::Error> {
        let res = UnmapViewOfFile(addr as *const c_void);
        if res == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub unsafe fn close(id: ShmId) -> Result<(), io::Error> {
        let res = CloseHandle(id);
        if res == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
    pub unsafe fn destroy(id: &str) -> Result<(), io::Error> {
        let name_utf16 = to_utf16(id);
        let shm_id = OpenFileMappingW(FILE_MAP_ALL_ACCESS, 0, name_utf16.as_ptr());
        if shm_id == null_mut() {
            return Err(io::Error::last_os_error());
        }
        let res = CloseHandle(shm_id);
        if res == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    let size = 4096;
    let id = "my_shared_memory";

    // Check arguments
    match std::env::args().nth(1).as_deref() {
        Some("--read") => {
            // Read-only mode
            let (addr, shm_id) = unsafe { platform::open(size, id)? };
            let data: &[u8] = unsafe { std::slice::from_raw_parts(addr, size) };
            println!("Reading from shared memory at: {:p}", addr);
            println!("Read data: {:?}", &data[0..3]);
            
            // Cleanup
            unsafe {
                platform::unmap(addr, size)?;
                platform::close(shm_id)?;
            }
        }
        None => {
            // Create the shared memory region
            let (addr, shm_id) = unsafe { platform::create(size, id)? };

            // Write some data to shared memory
            let data: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(addr, size) };
            data[0] = 1;
            data[1] = 2;
            data[2] = 3;

            println!("Shared memory created at: {:p} with id: {:?}", addr, shm_id);
            println!("Data Written: {:?}", &data[0..3]);
            println!("Press enter to close the shared memory.");
            std::io::stdin().read_line(&mut String::new()).unwrap();

            // Cleanup
            unsafe {
                platform::unmap(addr, size)?;
                platform::close(shm_id)?;
                platform::destroy(id)?;
            }
            println!("Shared memory closed and destroyed.");
        }
        _ => {
            println!("Usage: {} [--read]", std::env::args().next().unwrap());
            return Ok(());
        }
    }

    Ok(())
}
