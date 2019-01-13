.section ".text.start"

.global _start

_start:
LDR sp, =stack_top
BL entry_point
B .