# start memory address
.start 0x01020304

.label loop
    # label jump
    goto loop

# absolute jump
jmp 0xfafbfcfd
