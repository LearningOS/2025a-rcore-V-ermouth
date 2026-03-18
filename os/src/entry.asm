    # 定义一个代码段，名为.text.entry，专门放入口代码
    .section .text.entry

    # 把_start符号导出为全局符号，让链接器能看到，作为程序入口
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

    # 定义bss段，专门放栈空间（bss段是未初始化的内存，不会占用ELF文件体积）
    .section .bss.stack

    # 导出栈的下界符号
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    
    # 导出栈的上界符号
    .globl boot_stack_top
boot_stack_top: