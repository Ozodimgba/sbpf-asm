.globl e
e:
lddw r1, hello_message

// set message len
mov64 r2, 12 // 12 not 13 because rodata allocated 12 bytes

call sol_log_

mov64 r0, 0

exit

.section .rodata
  hello_message: .ascii "Hello Solana"
  // .align 8 // tried force allignment to read 13th byte

// external function declartion for solana log syscall
.extern sol_log_