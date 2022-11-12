    global _start

    section .text
_start:
    mov rax, 2
    cmp rax, 3
    je Junk
    mov rax, After
    jmp rax
Junk:
    db 0xf
After:
    mov rax, 1      ; x86_64 linux sys_write
    mov rdi, 1      ; stdout
    mov rsi, message
    mov rdx, 8      ; length of message
    syscall
    mov rax, 60     ; x86_64 linux sys_exit
    xor rdi, rdi    ; return code 0
    syscall

    section .data
message: db `tricked\n`
