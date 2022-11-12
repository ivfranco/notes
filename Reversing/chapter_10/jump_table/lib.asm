    global encode

    section .text
encode:
    push ecx                        ; esp = -0x4
    mov edx,dword [esp+8]           ; edx = PARAM_1
    push ebx
    push ebp                        ; esp = -0xC
    mov ebp,dword [esp+14h]         ; ebp = PARAM_2
    push esi
    push edi                        ; esp = -0x14
    mov edi,dword [esp+10h]         ; edi = ecx
    xor eax,eax                     ; eax = 0
    xor ebx,ebx                     ; ebx = 0
reset:
    mov ecx,1                       ; ecx = 1
    lea ebx,[ebx]                   ; noop
jump_table:
    lea esi,[ecx-1]                 ; esi = ecx - 1
    cmp esi,8                       ; if (esi > 8)
    ja jump_table                   ;   jump to jump_table
    jmp dword [esi*4+TABLE]         ; jump to table[esi]
table_3:
    xor dword [edx],ebx             ; *(DWORD *)edx ^= ebx
enter_loop:
    add ecx,1                       ; ecx += 1
    jmp jump_table                  ; jump to table[ecx - 1], table_1 or table_4
table_1:
    mov edi,dword [edx]             ; edi = *(DWORD *)edx
    add ecx,1                       ; ecx = 3
    jmp jump_table                  ; jump to table_2
table_0:
    cmp ebp,3                       ; if (ebp > 3)
    ja enter_loop
    mov ecx,9                       ; ecx = 9
    jmp jump_table                  ; jump to table_8
table_4:
    mov ebx,edi                     ; ebx = edi
    add ecx,1                       ; ecx = 6
    jmp jump_table                  ; jump to table_5
table_7:
    sub ebp,4                       ; ebp -= 4
    jmp reset
table_2:
    mov esi,dword [esp+20h]         ; esi = PARAM_3
    xor dword [edx],esi             ; *(DWORD *)edx ^= PARAM_3
    add ecx,1                       ; ecx = 4
    jmp jump_table                  ; jump to table_3
table_5:
    xor eax,edi                     ; eax ^= edi
    add ecx,1                       ; ecx = 7
    jmp jump_table                  ; jump to table_6
table_6:
    add edx,4                       ; edx += 4
    add ecx,1                       ; ecx += 1
    jmp jump_table                  ; jump to table_7
table_8:                            ; exit
    pop edi
    pop esi
    pop ebp
    pop ebx
    pop ecx
    ret

    section .data
TABLE:
    dd table_0
    dd table_1
    dd table_2
    dd table_3
    dd table_4
    dd table_5
    dd table_6
    dd table_7
    dd table_8