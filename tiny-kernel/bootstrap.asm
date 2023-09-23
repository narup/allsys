; Bootloader assembly code
;
start:
    mov ax, 07C0h
    mov ds, ax ; since we cannot load the value 070Ch directly into ds

    mov si, title_string 
    call print_string 

    mov si, message_string
    call print_string 

    call load_kernel_from_disk
    jmp 0900h:0000 ; gives the control to the kernel by jumping to its starting point 

; This block of code loads the kernel from the disk into the memory and to do that
; it uses the BIOS service 13h related to hard disks 

load_kernel_from_disk: 
    mov ax, 0900h
    mov es, ax

    mov ah, 02h ; service number to read the disk sector into the memory 
    mov al, 01h ; read only 1 sector since our kernel doesn't exceed 512 bytes 
    mov ch, 0h  ; number of track to use is 0
    mov cl, 02h ; second sector 
    mov dh, 0h  ; head number 
    mov dl, 80h ; 80h means hard disk 0, 81h means hard disk 1 
    mov bx, 0h  ; memory address where the content will be loaded to 
    int 13h     ; call interrupt for BIOS service related to hard disk 
    
    ; When the content is loaded successfully, the BIOS Service 13h:02h is 
    ; going to set the carry flag 45 to 0, otherwise, it sets the carry flag to 1

    jc kernel_load_error ; conditional jump if carry flag (CF) is 1. 

    ret  


kernel_load_error: 
    mov si, load_error_string 
    call print_string 

    jmp $

print_string:
    mov ah, 0Eh                  ; 0Eh represnts the service of printing a character on a screen

print_char:
    loadsb                  ; transfer the first character of the string to the register `al` and increase the value of the si by 1 
    cmp al, 0               ; if 0, we reached the end 
    je printing_finished 

    int 10h                 ; call video service
    jmp print_char 

printing_finished:
    mov al, 10d                 ; print new line, number 10 in ASCII 
    int 10h 

    ; Reading current cursor position using service 03h
    mov ah, 03h
    mov bh, 0
    int 10h 

    ; Move the cursor to the beginning using the service 02h
    mov ah, 02h
    mov dl, 0
    int 10h 

    ret

title_string db 'The bootloader of 539kernel.', 0 
message_string db 'The kernel is loading....', 0
load_error_string db 'The kernel cannot be loaded', 0 

; 2 bytes magic code so the firmware can recognize the content of the sector as a bootloader 
times 510-($-$$) db 0 ; fill the empty space between the bootloader and the magic code with 0. The expression ($ - $$) gives the size of the bootloader code
dw 0xAA55             ; magic code 