    ld sp, 0xFFFE
    ld a, 0xCE
    ld hl, 0x8010
    ld b, 0x04
    ld c, a
    push bc
    rl c
    rla
    pop bc
    rl c
    rla
    dec b
    jr nz, -11
    ld [hl+], a
    inc hl
    ld [hl+], a
    inc hl
    halt