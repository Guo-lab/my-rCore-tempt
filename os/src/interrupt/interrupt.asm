# 我们将会用一个宏来用循环保存寄存器。这是必要的设置
.altmacro
#
# 寄存器宽度对应的字节数
.set    REG_SIZE, 8
#
# Context 的大小: 32 + 1 + 1
.set    CONTEXT_SIZE, 34
#
#
#
#
# ###########################################################
#
# 宏：将寄存器存到栈上__REG_SIZE(sp)
.macro SAVE reg, offset
    sd \reg, \offset * 8(sp)
.endm
#
# 宏：将 n 号寄存器保存在第 n 个位置
.macro SAVE_N n
    SAVE x\n, \n
.endm
#
#
#
#
#
# 宏：将寄存器从栈中取出__REG_SIZE(sp)
.macro LOAD reg, offset
    ld \reg, \offset * 8(sp)
.endm
#
#
# 宏：将 n 号寄存器从第 n 个位置取出
.macro LOAD_N n
    LOAD x\n, \n
.endm
#
#
#
#
#

# ###########################################################
    .section .text
    .globl __interrupt
# 进入中断
# 保存 Context 并且进入 Rust 中的中断处理函数 interrupt::handler::handle_interrupt()
#

__interrupt:
    # 在内核栈开辟 Context 的空间__CONTEXT_SIZE * REG_SIZE
    addi    sp, sp, -34*8
    #
    # 保存通用寄存器，除了 x0（固定为 0）
    SAVE    x1, 1
    #
    # 将本来的栈地址 sp（即 x2）保存 write into Location 2
    addi    x1, sp, 34*8
    SAVE    x1, 2
    #
    # 保存 x3 至 x31
    .set    n, 3
    .rept   29
        SAVE_N  %n
        .set    n, n + 1
    .endr

    # 取出 CSR 并保存
    csrr    s1, sstatus
    csrr    s2, sepc
    SAVE    s1, 32
    SAVE    s2, 33
    #
    # 调用 handle_interrupt，传入参数
    # context: &mut Context
    mv      a0, sp
    #
    # scause: Scause
    csrr    a1, scause
    #
    # stval: usize
    csrr    a2, stval
    jal handle_interrupt
    # ###########################################################

    .globl __restore
# 离开中断
# 接下来从 Context 中恢复所有寄存器
# 最后跳转至恢复的 sepc 的位置

__restore:
    # 恢复 CSR
    LOAD    s1, 32
    LOAD    s2, 33
    csrw    sstatus, s1
    csrw    sepc, s2


    # 恢复通用寄存器
    LOAD    x1, 1
    #
    # 恢复 x3 至 x31
    .set    n, 3
    .rept   29
        LOAD_N  %n
        .set    n, n + 1
    .endr
    #

    # 恢复 sp（又名 x2）这里最后恢复是为了上面可以正常使用 LOAD 宏 ?????
    LOAD    x2, 2
    sret

# ###########################################################
# 终端发生，循环保存 Reg ，保存调用处理函数：分配空间，【栈sp不断使用告知CPU其他寄存器地址】
# 状态 sStatus；中断地址 sepc 刷新；handleinterrupt INPUT sp scause stval
# 恢复时 CSR（LOAD取出 s1 s2 sstatus sepc） 恢复通用x1 x3 EVENTUALLY sp！
