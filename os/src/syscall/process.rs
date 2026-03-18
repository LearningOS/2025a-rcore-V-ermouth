//! App management syscalls
use crate::batch::run_next_app;

/// 任务结束退出系统调用
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}
