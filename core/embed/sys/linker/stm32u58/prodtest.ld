ENTRY(reset_handler)

MEMORY {
  FLASH  (rx)  : ORIGIN = FIRMWARE_START, LENGTH = FIRMWARE_MAXSIZE

  MAIN_RAM (wal) : ORIGIN = MAIN_RAM_START, LENGTH = MAIN_RAM_SIZE
  AUX1_RAM (wal) : ORIGIN = AUX1_RAM_START, LENGTH = AUX1_RAM_SIZE
  AUX2_RAM (wal) : ORIGIN = AUX2_RAM_START, LENGTH = AUX2_RAM_SIZE
  BOOT_ARGS (wal) : ORIGIN = BOOTARGS_START, LENGTH = BOOTARGS_SIZE
  FB1_RAM (wal) : ORIGIN = FB1_RAM_START, LENGTH = FB1_RAM_SIZE
  FB2_RAM (wal) : ORIGIN = FB2_RAM_START, LENGTH = FB2_RAM_SIZE
}

_stack_section_start = ADDR(.stack);
_stack_section_end = ADDR(.stack) + SIZEOF(.stack);

_data_section_loadaddr = LOADADDR(.data);
_data_section_start = ADDR(.data);
_data_section_end = ADDR(.data) + SIZEOF(.data);

_bss_section_start = ADDR(.bss);
_bss_section_end = ADDR(.bss) + SIZEOF(.bss);

_confidential_section_loadaddr = LOADADDR(.confidential);
_confidential_section_start = ADDR(.confidential);
_confidential_section_end = ADDR(.confidential) + SIZEOF(.confidential);

_bootargs_ram_start = BOOTARGS_START;
_bootargs_ram_end = BOOTARGS_START + BOOTARGS_SIZE;

_codelen = SIZEOF(.flash) + SIZEOF(.data) + SIZEOF(.confidential);

SECTIONS {
  .vendorheader : ALIGN(4) {
    KEEP(*(.vendorheader))
  } >FLASH AT>FLASH

  .header : ALIGN(4) {
    KEEP(*(.header));
  } >FLASH AT>FLASH

  .flash : ALIGN(CODE_ALIGNMENT) {
    KEEP(*(.vector_table));
    . = ALIGN(4);
    *(.text*);
    . = ALIGN(4);
    *(.rodata*);
    . = ALIGN(4);

    _prodtest_cli_cmd_section_start = .;
    KEEP(*(.prodtest_cli_cmd))
    _prodtest_cli_cmd_section_end = .;
    . = ALIGN(4);

    _unit_test_section_start = .;
    KEEP(*(.unit_test))
    _unit_test_section_end = .;

    . = ALIGN(512);
  } >FLASH AT>FLASH

  .data : ALIGN(4) {
    *(.data*);
    . = ALIGN(512);
  } >AUX1_RAM AT>FLASH

  /DISCARD/ : {
    *(.ARM.exidx*);
  }

  .bss : ALIGN(4) {
    *(.bss*);
    . = ALIGN(4);
  } >AUX1_RAM

  .buf : ALIGN(4) {
    *(.buf*);
    . = ALIGN(4);
    *(.no_dma_buffers*);
    . = ALIGN(4);
  } >AUX2_RAM

  .stack : ALIGN(8) {
    . = 16K; /* Overflow causes UsageFault */
  } >MAIN_RAM

  .confidential : ALIGN(512) {
    *(.confidential*);
    . = ALIGN(512);
  } >MAIN_RAM AT>FLASH

  .fb1 : ALIGN(4) {
    *(.fb1*);
    . = ALIGN(4);
  } >FB1_RAM

  .fb2 : ALIGN(4) {
    *(.fb2*);
    . = ALIGN(4);
  } >FB2_RAM

  .boot_args : ALIGN(8) {
    *(.boot_command*);
    . = ALIGN(8);
    *(.boot_args*);
    . = ALIGN(8);
  } >BOOT_ARGS
}
