    global main
    extern strlen
    extern printf

    section .text
main:
    enter 0x4, 0
; [esp-0x0]: char *exportName;
    mov eax, dword [ebp + 0x8]      ; argc
    cmp eax, 2
    jb  exit
    mov eax, dword [ebp + 0xC]      ; argv
    mov eax, dword [eax + 0x4]      ; argv[1]
    mov dword [esp], eax
    push eax
    call strlen                     ; eax = strlen(argv[1])
    add esp, 0x4
    xor esi, esi
    xor ebx, ebx
    mov ecx, eax
    mov eax, 1
    mov edx, 0x25
loop:
    MOV EBX,DWORD [esp]
    mov ebx, dword [ebx]
    ; MOVSX EDX,BYTE PTR [EAX+40351F]
    SUB EBX,EDX
    IMUL EBX,EDX
    MOV ESI,EBX
    SUB EBX,EAX
    ADD EBX,0x4353543
    ADD ESI,EBX
    XOR ESI,EDX
    MOV EAX,4
    mov edx, 0x65
    DEC ECX
    JNZ loop
print:
    push 0
    push esi
    push fmt
    call printf                     ; printf("%u\n", esi)
    add esp, 0xC
exit:
    add esp, 0x4
    xor eax, eax
    leave
    ret

    section .data
fmt: db `%u\n`, 0
