use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use url::Url;

use crate::readers::MdxReader;

// 定义一个结构体来持有 MdxReader 实例
#[repr(C)]
pub struct MdxHandle {
    _private: [u8; 0],
}

// 将实际的 MdxReader 包装在 Box 中，通过裸指针传递给 C
type MdxReaderBox = Box<MdxReader<std::fs::File>>;

// 打开 MDX 文件并返回句柄
#[no_mangle]
pub extern "C" fn mdx_open(file_path: *const c_char, device_id: *const c_char) -> *mut MdxHandle {
    if file_path.is_null() || device_id.is_null() {
        return std::ptr::null_mut();
    }

    let file_path_cstr = unsafe { CStr::from_ptr(file_path) };
    let device_id_cstr = unsafe { CStr::from_ptr(device_id) };

    let file_path_str = match file_path_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let device_id_str = match device_id_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let file_url = match Url::from_file_path(file_path_str) {
        Ok(url) => url,
        Err(_) => return std::ptr::null_mut(),
    };

    match MdxReader::from_url(&file_url, device_id_str) {
        Ok(reader) => {
            let reader_box: MdxReaderBox = Box::new(reader);
            Box::into_raw(reader_box) as *mut MdxHandle
        }
        Err(_) => std::ptr::null_mut(),
    }
}

// 关闭 MDX 文件并释放资源
#[no_mangle]
pub extern "C" fn mdx_close(handle: *mut MdxHandle) {
    if !handle.is_null() {
        let _ = unsafe { Box::from_raw(handle as MdxReaderBox) };
    }
}

// 查找关键词并返回 HTML 定义
#[no_mangle]
pub extern "C" fn mdx_lookup(handle: *mut MdxHandle, keyword: *const c_char) -> *mut c_char {
    if handle.is_null() || keyword.is_null() {
        return std::ptr::null_mut();
    }

    let keyword_cstr = unsafe { CStr::from_ptr(keyword) };
    let keyword_str = match keyword_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let reader = unsafe { &mut *(handle as MdxReaderBox) };

    match reader.lookup(keyword_str) {
        Ok(key_index) => {
            match reader.get_html(&key_index) {
                Ok(html) => {
                    match CString::new(html) {
                        Ok(c_string) => c_string.into_raw(),
                        Err(_) => std::ptr::null_mut(),
                    }
                }
                Err(_) => std::ptr::null_mut(),
            }
        }
        Err(_) => std::ptr::null_mut(),
    }
}

// 释放由 mdx_lookup 返回的字符串
#[no_mangle]
pub extern "C" fn mdx_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

// 检查关键词是否存在
#[no_mangle]
pub extern "C" fn mdx_has_key(handle: *mut MdxHandle, keyword: *const c_char) -> i32 {
    if handle.is_null() || keyword.is_null() {
        return 0; // false
    }

    let keyword_cstr = unsafe { CStr::from_ptr(keyword) };
    let keyword_str = match keyword_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    let reader = unsafe { &mut *(handle as MdxReaderBox) };

    match reader.lookup(keyword_str) {
        Ok(_) => 1, // true
        Err(_) => 0, // false
    }
}

// 获取元数据 - 词典标题
#[no_mangle]
pub extern "C" fn mdx_get_title(handle: *mut MdxHandle) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    let reader = unsafe { &*(handle as MdxReaderBox) };
    
    match CString::new(reader.meta().db_info.title.clone()) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

// 获取元数据 - 词典描述
#[no_mangle]
pub extern "C" fn mdx_get_description(handle: *mut MdxHandle) -> *mut c_char {
    if handle.is_null() {
        return std::ptr::null_mut();
    }

    let reader = unsafe { &*(handle as MdxReaderBox) };
    
    match CString::new(reader.meta().db_info.description.clone()) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}