use std::ffi::CString;

use crate::error::NappguiError;
use crate::types::CookiePolicy;

/// HTTP request
pub struct Request {
    pub(crate) inner: *mut nappgui_sys::Http,
}

impl Request {
    /// Create an HTTP session.
    pub fn new(host: &str, port: u16) -> Self {
        let host = CString::new(host).unwrap();
        let inner = unsafe { nappgui_sys::http_create(host.as_ptr(), port) };
        Self { inner }
    }

    /// Create an HTTPS session.
    pub fn new_secure(host: &str, port: u16) -> Self {
        let host = CString::new(host).unwrap();
        let inner = unsafe { nappgui_sys::http_secure(host.as_ptr(), port) };
        Self { inner }
    }

    /// Removes previously assigned HTTP headers.
    pub fn clear_headers(&mut self) {
        unsafe { nappgui_sys::http_clear_headers(self.inner) }
    }

    /// Add a header to the HTTP request. Return TRUE if the header could be added to the request.
    pub fn add_header(&mut self, name: &str, value: &str) -> bool {
        let name = CString::new(name).unwrap();
        let value = CString::new(value).unwrap();
        unsafe { nappgui_sys::http_add_header(self.inner, name.as_ptr(), value.as_ptr()) != 0 }
    }

    /// Change the cookie policy.
    ///
    /// # Remarks
    ///
    /// The set policy will be applied in future calls to http_get or similar. By default, cookies are enabled.
    pub fn cookies_set_policy(&mut self, policy: CookiePolicy) {
        unsafe { nappgui_sys::http_cookies_policy(self.inner, policy.into()) }
    }

    /// Reload the cookie cache.
    ///
    /// # Remarks
    /// We must call this function before accessing the content of the cookies. If the policy is Off,
    /// the cookie cache will always be empty.
    pub fn cookies_reload(&mut self) {
        unsafe { nappgui_sys::http_cookies_reload(self.inner) }
    }

    /// Returns the number of cookies associated with this session.
    pub fn cookies_size(&self) -> u32 {
        unsafe { nappgui_sys::http_cookies_size(self.inner) }
    }

    /// Returns the name of a cookie associated with this session.
    ///
    /// # Arguments
    /// * `index` - Cookie index (less than `cookies_size`).
    ///
    /// # Remarks
    /// Call before cookies_reload.
    pub fn cookie_name(&self, index: u32) -> &str {
        unsafe {
            let ptr = nappgui_sys::http_cookie_name(self.inner, index);
            if ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Returns the value of a cookie associated with this session.
    ///
    /// # Arguments
    /// * `index` - Cookie index (less than `cookies_size`).
    ///
    /// # Remarks
    /// Call before cookies_reload.
    pub fn cookie_value(&self, index: u32) -> &str {
        unsafe {
            let ptr = nappgui_sys::http_cookie_value(self.inner, index);
            if ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Returns the value of a cookie, given its name.
    ///
    /// # Arguments
    /// * `name` - Cookie name.
    ///
    /// # Returns
    /// Cookie value or empty string if the cookie does not exist.
    ///
    /// # Remarks
    /// Call before cookies_reload.
    pub fn cookie_search(&self, name: &str) -> &str {
        let name = CString::new(name).unwrap();
        unsafe {
            let ptr = nappgui_sys::http_cookie_search(self.inner, name.as_ptr());
            if ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Delete a cookie associated with this session.
    ///
    /// # Arguments
    /// * `name` - Cookie name.
    pub fn cookie_delete(&mut self, name: &str) {
        let name = CString::new(name).unwrap();
        unsafe { nappgui_sys::http_cookie_delete(self.inner, name.as_ptr()) }
    }

    /// Delete all cookies associated with this session.
    pub fn cookie_delete_all(&mut self) {
        unsafe { nappgui_sys::http_cookie_delete_all(self.inner) }
    }

    /// Make a GET type request.
    ///
    /// # Arguments
    /// * `path` - Resource path.
    /// * `data` - Data to add to the body of the request. It can be empty.
    ///
    /// # Returns
    /// `Ok(())` if the request was carried out correctly, or `Err(NappguiError)` with the cause.
    pub fn get(&mut self, path: &str, data: &[u8]) -> Result<(), NappguiError> {
        let path = CString::new(path).unwrap();
        let mut error: nappgui_sys::ierror_t = 0;
        let ok =
            unsafe { nappgui_sys::http_get(self.inner, path.as_ptr(), data.as_ptr(), data.len() as u32, &mut error) };
        if ok != 0 {
            Ok(())
        } else {
            Err(NappguiError::from_ierror_t(error))
        }
    }

    /// Make a POST type request.
    ///
    /// # Arguments
    /// * `path` - Resource path.
    /// * `data` - Data to add to the body of the request. It can be empty.
    ///
    /// # Returns
    /// `Ok(())` if the request was carried out correctly, or `Err(NappguiError)` with the cause.
    pub fn post(&mut self, path: &str, data: &[u8]) -> Result<(), NappguiError> {
        let path = CString::new(path).unwrap();
        let mut error: nappgui_sys::ierror_t = 0;
        let ok =
            unsafe { nappgui_sys::http_post(self.inner, path.as_ptr(), data.as_ptr(), data.len() as u32, &mut error) };
        if ok != 0 {
            Ok(())
        } else {
            Err(NappguiError::from_ierror_t(error))
        }
    }

    /// Make a PUT type request.
    ///
    /// # Arguments
    /// * `path` - Resource path.
    /// * `data` - Data to add to the body of the request. It can be empty.
    ///
    /// # Returns
    /// `Ok(())` if the request was carried out correctly, or `Err(NappguiError)` with the cause.
    pub fn put(&mut self, path: &str, data: &[u8]) -> Result<(), NappguiError> {
        let path = CString::new(path).unwrap();
        let mut error: nappgui_sys::ierror_t = 0;
        let ok =
            unsafe { nappgui_sys::http_put(self.inner, path.as_ptr(), data.as_ptr(), data.len() as u32, &mut error) };
        if ok != 0 {
            Ok(())
        } else {
            Err(NappguiError::from_ierror_t(error))
        }
    }

    /// Make a PATCH type request.
    ///
    /// # Arguments
    /// * `path` - Resource path.
    /// * `data` - Data to add to the body of the request. It can be empty.
    ///
    /// # Returns
    /// `Ok(())` if the request was carried out correctly, or `Err(NappguiError)` with the cause.
    pub fn patch(&mut self, path: &str, data: &[u8]) -> Result<(), NappguiError> {
        let path = CString::new(path).unwrap();
        let mut error: nappgui_sys::ierror_t = 0;
        let ok =
            unsafe { nappgui_sys::http_patch(self.inner, path.as_ptr(), data.as_ptr(), data.len() as u32, &mut error) };
        if ok != 0 {
            Ok(())
        } else {
            Err(NappguiError::from_ierror_t(error))
        }
    }

    /// Make a DELETE type request.
    ///
    /// # Arguments
    /// * `path` - Resource path.
    /// * `data` - Data to add to the body of the request. It can be empty.
    ///
    /// # Returns
    /// `Ok(())` if the request was carried out correctly, or `Err(NappguiError)` with the cause.
    pub fn delete(&mut self, path: &str, data: &[u8]) -> Result<(), NappguiError> {
        let path = CString::new(path).unwrap();
        let mut error: nappgui_sys::ierror_t = 0;
        let ok = unsafe {
            nappgui_sys::http_delete(self.inner, path.as_ptr(), data.as_ptr(), data.len() as u32, &mut error)
        };
        if ok != 0 {
            Ok(())
        } else {
            Err(NappguiError::from_ierror_t(error))
        }
    }

    /// Returns the response code of an HTTP request.
    pub fn response_status(&self) -> u32 {
        unsafe { nappgui_sys::http_response_status(self.inner) }
    }

    /// Returns the protocol used by the HTTP server.
    pub fn response_protocol(&self) -> &str {
        unsafe {
            let ptr = nappgui_sys::http_response_protocol(self.inner);
            if ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Returns the response message from the HTTP server.
    pub fn response_message(&self) -> &str {
        unsafe {
            let ptr = nappgui_sys::http_response_message(self.inner);
            if ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Returns the number of response headers for an HTTP request.
    pub fn response_size(&self) -> u32 {
        unsafe { nappgui_sys::http_response_size(self.inner) }
    }

    /// Returns the name of the response header of an HTTP request.
    ///
    /// # Arguments
    /// * `index` - The header index (0, size-1).
    pub fn response_name(&self, index: u32) -> &str {
        unsafe {
            let ptr = nappgui_sys::http_response_name(self.inner, index);
            if ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Returns the value of the response header of an HTTP request.
    ///
    /// # Arguments
    /// * `index` - The header index (0, size-1).
    pub fn response_value(&self, index: u32) -> &str {
        unsafe {
            let ptr = nappgui_sys::http_response_value(self.inner, index);
            if ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Returns the value of a response header from an HTTP request.
    ///
    /// # Arguments
    /// * `name` - The name of the desired header.
    ///
    /// # Returns
    /// The value of the header, or empty string if the header does not exist.
    pub fn response_header(&self, name: &str) -> &str {
        let name = CString::new(name).unwrap();
        unsafe {
            let ptr = nappgui_sys::http_response_header(self.inner, name.as_ptr());
            if ptr.is_null() {
                ""
            } else {
                std::ffi::CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Returns the response body of an HTTP request.
    ///
    /// # Returns
    /// `Ok(Vec<u8>)` with the response body bytes, or `Err(NappguiError)` with the cause.
    pub fn response_body(&self) -> Result<Vec<u8>, NappguiError> {
        todo!()
    }

    /// Make a direct request for a Web resource.
    ///
    /// # Arguments
    /// * `url` - Resource URL.
    ///
    /// # Returns
    /// `Ok((Vec<u8>, u32))` with the response body bytes and the HTTP status code,
    /// or `Err(NappguiError)` with the cause.
    pub fn dget(_url: &str) -> Result<(Vec<u8>, u32), NappguiError> {
        todo!()
    }

    /// Checks if a Web resource is available/accessible.
    ///
    /// # Arguments
    /// * `url` - Resource URL.
    ///
    /// # Returns
    /// `true` if the resource is accessible, `false` otherwise.
    pub fn exists(url: &str) -> bool {
        let url = CString::new(url).unwrap();
        unsafe { nappgui_sys::http_exists(url.as_ptr()) != 0 }
    }
}

impl Drop for Request {
    fn drop(&mut self) {
        unsafe { nappgui_sys::http_destroy(&mut self.inner) };
    }
}
