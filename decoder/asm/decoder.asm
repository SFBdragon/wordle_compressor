; Assemble using:
;    nasm -f bin -o decoder decoder.asm
[bits 64]

window_len equ 1 << 10

vaddr   equ 0x08048000
        org vaddr                                       ; set base address for asm code to vaddr

; elf header
        db          0x7f,'ELF',2,1,1,0,0,0,0,0,0,0,0,0  ; e_ident
        dw          2                                   ; e_type
        dw          0x3e                                ; e_machine
        dd          1                                   ; e_version
        dq          _start                              ; e_entry
        dq          phdr - $$                           ; e_phoff
        dq          0                                   ; e_shoff
        dd          0                                   ; e_flags
        dw          64                                  ; e_ehsize
        dw          0x38                                ; e_phentsize
        dw          1                                   ; e_phnum
        dw          0x40                                ; e_shentsize
        dw          0                                   ; e_shnum
        dw          0                                   ; e_shstrndx

phdr:
        dd          1                                   ; p_type
        dd          7                                   ; p_flags      ; exec, write, read
        dq          0                                   ; p_offset     ; load entire file
        dq          vaddr                               ; p_vaddr
        dq          vaddr                               ; p_paddr
        dq          eof - vaddr                         ; p_filesz     ; load entire file
        dq          eof - vaddr + window_len            ; p_memsz      ; include extra amount of memory for lzss window
        dq          0x1000                              ; p_align



; get the args off the stack
; find the index
; begin decoding from the start to the index
;    extract bits until a huffman code match is found, decode
;    if decode is a ;, grab the next entry in the matches, and pull the matched data out of the window, if any of those are 0/1/2 handle accordingly
;    else if decode is a 0/1/2 update the trackers thereof, queue into window
;    else queue the decode into the window
; at the index, get the word based on the 0/1/2 index tracking and extra 2 chars, lookup into the bitmap to get if is answer, return in an 8 byte reg



_start:                                         ; program starts here
        pop         rcx                         ; grab argc off the stack
        cmp         rcx, 2                      ; test if only one arg has been provided
        je          no_arg_err                  ; err if not the case
        mov         rdi, arg_err_msg
        mov         rsi, arg_err_msg_len
        jne         msg_n_exit
no_arg_err:
        pop         rdi                         ; pop the first argv
        mov         rsi, 0                      ; set the sentinel for len_until to zero
        call len_until                          ; find arg length, rdi is already set to string ptr, result is in rax
        
        cmp         byte [rdi], 'a'             ; test the first character against 'a'
        jl          parse_num_arg               ; if less than 'a', jump to numeric parse

        mov         qword [arg_mem], rdi        ; move the string ptr into arg_mem
        mov         r11, word_test              ; move the instruction ptr for word testing into r11


        mov         rsi, rax
        jmp         msg_n_exit

        jmp         arg_parsed
parse_num_arg:

        mov         rdi, arg_err_msg
        mov         rsi, rax
        jmp         msg_n_exit

        mov         r11, index_test             ; move the instruction ptr for index testing into r11

arg_parsed:

        pop         rax                         ; pop the first argv - it's the program file name
        pop         rdi
        ;pop         rsi

        mov         rsi, 0
        call        len_until
  
        mov         rax, 60                         ; syscall for exit
        mov         rdi, 0                          ; set exit code
        syscall


msg_n_exit:                                         ; rdi: message string ptr, rsi: message len
        mov         rdx, rsi                        ; message len
        mov         rsi, rdi                        ; message ptr
        mov         rax, 1                          ; syscall for write
        mov         rdi, 1                          ; file handle 1 is stdout
        syscall
        mov         rax, 60                         ; syscall for exit
        mov         rdi, 101                        ; set exit code
        syscall
  
len_until:                                          ; rdi: array ptr, rsi (sil): sentinel byte
        mov         rax, 0                          ; store running len in return register
    loop:
            cmp         sil, byte [rdi]             ; check if ptr contents and sentinel are equal
            je          len_until_ret               ; if so, return len, otherwise:
            inc         rdi                         ; inc ptr
            inc         rax                         ; inc running len
            jmp         loop                        ; loop
len_until_ret: ret

index_test:


word_test:




arg_mem:
        dq 1000000


arg_err_msg:
        db '1 arg only, word or index', 0xa
arg_err_msg_len equ $ - arg_err_msg

ans_bitmap:
        incbin "../answer_words_bitmap"
ans_bitmap_len equ $ - ans_bitmap

match_list:
        incbin "../lzss_matches"
match_list_len equ $ - match_list

blob:
        incbin "../words_i3_lzss_he"
blob_len equ $ - blob


eof:
window:
