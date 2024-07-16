OUTPUT_ARCH("riscv")
/* カーネルのエントリポイント */
ENTRY(_boot)

SECTIONS {
    /* ベースアドレス：SBIが0x80200000に配置されたプログラムを実行するようになっているため */
    . = 0x80200000;

    /* コード領域 */
    .text :{ 
        /* 必ず.text.bootセクションを先頭にする */
        KEEP(*(.text.boot));
        *(.text .text.*);
    }

    /* 定数データ領域。読み取り専用 */
    .rodata : ALIGN(4) {
        *(.rodata .rodata.*);
    }

    /* 読み書き可能データ領域 */
    .data : ALIGN(4) {
        *(.data .data.*);
    }

    /* 読み書き可能データ領域。.dataと違い初期値がゼロの変数を配置 */
    .bss : ALIGN(4) {
        __bss = .;
        *(.bss .bss.* .sbss .sbss.*);
        /* ブート時に使うスタック領域を配置 */
        __bss_end = .;
    }

    . = ALIGN(4);
    . += 128 * 1024; /* 128KB */
    __stack_top = .;
}