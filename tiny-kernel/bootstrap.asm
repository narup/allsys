start:
    mov ax, 07C0h
    mov ds, ax

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
    int 13h
    
    ; When the content is loaded successfully, the BIOS Service 13h:02h is 
    ; going to set the carry flag 45 to 0, otherwise, it sets the carry flag to 1

    jc kernel_load_error ; conditional jump if carry flag (CF) is 1. 

    ret 


kernel_load_error: 
    mov si, load_error_string 
    call print_string 

    jmp $