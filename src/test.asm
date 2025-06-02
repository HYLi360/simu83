    ld bc, 0xff12
    inc hl
    bit 7, h
    jp z, 0x00
    halt