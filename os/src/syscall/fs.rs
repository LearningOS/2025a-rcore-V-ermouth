//! File and filesystem-related syscalls

const FD_STDOUT: usize = 1;

/// # 系统输出
/// 
/// 从buf中读取len个字节输出到标准输出
/// 
/// return `len`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel: sys_write");
    match fd {
        FD_STDOUT => {
            // 从buf开始获取len个字节
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => {
            panic!("Unsupported fd in sys_write!");
        }
    }
}
