.text

.global _start
_start:
	movl $1, %eax		# exit sys call numver in syscall register
	movl $10, %ebx		# sets EBX register to hold 10 (EBX is also holding exit status)
	movl $11, %ecx		# sets ECX to specified val (below or eq to 10 will (jump) bigger than 10 woll ignore)

	cmpl %ecx, %ebx		# compares ECX(9) and EBX(10) registers
	jge end_block		# if condition meet jump to the end
	movl $0, %ebx		# if condition not met (no jump was performed) program will execut next instructions setting EBX to 0 (exit status)

end_block:				# is used to mark spot to jump to in jge instruction
	int $0x80			# runs syscall (EAX is still 1 so syscall is exti) with exit status based on EBX register
