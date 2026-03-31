.section .text
.global _start
_start:
    xor %rbp, %rbp
    mov 0(%rsp), %rdi
    lea 8(%rsp), %rsi
    call main
    mov %rax, %rdi
    call exit
.section .note.GNU-stack,"",@progbits
