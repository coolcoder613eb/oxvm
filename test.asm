# start memory address
.start 0x00ff

.label loop
    # label jump
    goto loop

# absolute jump
jmp 0xff00
