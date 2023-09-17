start:
    mov ax, 07C0h
    mov ds, ax

    mov si, title_string 
    call print_string 

    mov si, message_string
    call print_string 

    call load_kernel_from_disk
    jmp 0900h:0000 

; This block of code loads the kernel from the disk into the memory and to do that
; it uses the BIOS service 13h related to hard disks 

load_kernel_from_disk: 
    mov ax, 0900h
    mov es, ax

    mov ah, 02h ; service number to read the disk sector into the memory 
    mov al, 01h ; read only 1 sector since our kernel doesn't exceed 512 bytes 
    mov ch, 0h
    mov cl, 02h
    mov dh, 0h
    mov dl, 80h
    mov bx, 0h
    int 13h
    
    jc kernel_load_error

    ret 
