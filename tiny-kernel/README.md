### NOTES:

#### Tiny kernel implementation https://539kernel.com/

- NASM command

    ``` 
        nasm -f <format> <filename> [-o <output>]
    ```
- How registers are divided in x86

    ![Alt text](image.png)

- Default binary format in Linux is ELF (Executable and Linkable Format). Mach-O is mac based and PE (Portable Execution) is used by windows
- i386 is another name for x86 processor architecture 
- Fat ELF is used to combine binaries for multiple architectures 
- Bootloader (piece of code) loads kernel of the OS from the disk to the main memory (RAM) and gives kernel the control of the computer 
- The size of the bootloader is 512 bytes
- Firmware loads the bootloader from the boot sector in the hard disk to memory address 07C0h. BIOS (Basic Input Output System) in IBM-compatible computers 
- BIOS services are divided into categories - video, keyboard, disk, and so on identified with interrupt number 
- Calling interrupt in assembly is simple `int 0x10`
- x86 processor uses Von Neuman architecture - both code and the data are stored in the same memory and processor uses this memory to read the instructions 
- Each memory location in the main memory has a unique address. Each instruction of the loaded program in the memory has an address
- Program counter keep tracks of the memory pointer of an executing program. It's also called instruction pointer IP in 16-bit and EIP in 32-bit
- Running program has a stack which is a region of the program's memory.  
- FLAGS register is a special register in x86, holds the status of the processor. Each bit has a special purpose. Bit-0 is called carry flag (CF) and bit-6 is known as zero flag (Zero Flag)
- In NASM, each line of assembly code has the format: `label: instruction operands` 
- 1 byte == 8-bits, 1 word == 2 bytes == 16-bit, doubleword == 4 bytes == 32-bit 
- Expression is a part of the code that evaluates a value. For example - x + 1, x == 5 
- Statement is a part of the code that performs some action, x = y + 1 
- Based on how the hard disk work where there are plates and each plate has tracks divided into sectors. So, when BIOS loads a bootloader, at first the arm will seek the track number 0 and sector 0, the content of the sector 0 is loaded in the main memory 
- 