.global output
output:
    movq $0x0000001, %rax
    syscall
    retq
