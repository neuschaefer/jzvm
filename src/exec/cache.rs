// SPDX-License-Identifier: LGPL-2.1
pub fn flush_slice(slice: &[u8]) {
    cfg_if::cfg_if! {
        if #[cfg(all(target_arch = "arm", any(target_os = "linux", target_os = "android")))] {
            unsafe {
                let arm_nr_cacheflush = 0xf0002;
                let begin: usize = slice.as_ptr() as usize;
                let end: usize  = slice.as_ptr().add(slice.len()) as usize;
                libc::syscall(arm_nr_cacheflush, begin, end, 0);
            }
        } else {
            // do nothing
            let _ = slice;
        }
    }
}
