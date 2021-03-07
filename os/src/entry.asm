# OS启动时所需指令以及字段
#
# 在 linker.ld 中将程序入口设置为了 _start，在这里我将填充这个标签
# 它将会执行一些必要操作，然后跳转至我们用 rust 编写的入口函数
#
# 关于RISC-V下汇编语言Reference: https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md
# %hi 表示取 [12,32)高十二位，%lo 表示取 [0,12)低十二位


# ****************************************************************
    .section .text.entry
    .globl _start



# 目前 _start 的功能：将预留的栈空间写入 $sp，然后跳转至 rust_main
_start:
    # 加载栈的虚拟地址(栈指针寄存器sp改为栈段结束地址，高地址初始栈顶)
    #
    la sp, boot_stack_top
    call rust_main
    #    
    # 跳转至 rust_main，正式进入内核
    #
    #
    # 回忆：bss 段是 ELF 文件中只记录长度，而全部初始化为 0 的一段内存空间
    # 这里声明字段 .bss.stack 作为操作系统启动时的栈
    #
    .section .bss.stack
    .global boot_stack


boot_stack:
    # 16K---启动栈大小
    .space 4096 * 16
    .global boot_stack_top


boot_stack_top:
    # 栈结尾

