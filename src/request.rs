use windows::core::PCSTR;
use windows::Win32::Networking::WinInet::*;

struct InternetHandleWrapper(*const core::ffi::c_void);

impl Drop for InternetHandleWrapper {
    fn drop(&mut self) {
        unsafe {
            if !self.0.is_null() {
                let _ = InternetCloseHandle(self.0);
            }
        }
    }
}

pub fn download(url: &str) -> Option<Vec<u8>> {
    let mut response = Vec::new();
    let mut url_raw = url.as_bytes().to_vec();
    url_raw.push(0);

    unsafe {
        let internet_handle = InternetHandleWrapper(InternetOpenA(
            PCSTR::from_raw("WinInet Download\0".as_bytes().as_ptr()),
            INTERNET_OPEN_TYPE_PRECONFIG.0,
            PCSTR::null(),
            PCSTR::null(),
            0,
        ));

        if internet_handle.0.is_null() {
            return None;
        }

        let connect_handle = InternetHandleWrapper(InternetOpenUrlA(
            internet_handle.0,
            PCSTR::from_raw(url_raw.as_ptr()),
            None,
            INTERNET_FLAG_RELOAD,
            None,
        ));

        if connect_handle.0.is_null() {
            return None;
        }

        let mut status_code: u32 = 0;
        let mut status_code_size = std::mem::size_of_val(&status_code) as u32;
        
        if HttpQueryInfoA(
            connect_handle.0,
            HTTP_QUERY_STATUS_CODE | HTTP_QUERY_FLAG_NUMBER,
            Some(&mut status_code as *mut u32 as *mut core::ffi::c_void),
            &mut status_code_size,
            None,
        )
        .is_err()
        {
            return None;
        }
        
        if status_code != 200 {
            return None;
        }

        let mut buffer = [0u8; 65535];
        let mut bytes_read: u32 = 0;

        loop {
            if InternetReadFile(
                connect_handle.0,
                buffer.as_mut_ptr() as *mut core::ffi::c_void,
                buffer.len() as u32,
                &mut bytes_read,
            )
            .is_err()
            {
                return None;
            }

            if bytes_read == 0 {
                break;
            }

            response.extend_from_slice(&buffer[..bytes_read as usize]);
        }
    }

    return Some(response);
}
