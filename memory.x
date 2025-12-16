/* 
This linker script defines the memory layout for the MPS2 AN385 board used in QEMU.
It specifies the location and size of FLASH and RAM.
*/
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 512K
}

/* The entry point of the application */
ENTRY(reset);

/* Define a section for the stack */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
