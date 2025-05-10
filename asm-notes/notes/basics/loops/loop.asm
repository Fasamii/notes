.global _start
.section .text
_start:
	movl $1, %eax		# sets syscall reg to exit
	movl $1, %ebx		# moves 10 into EBX registe (first arg register)
	movl $10, %ecx		# moves 1 into ECX (counter register)

loop:
	addl $1, %ebx		# adds 1 to ECX
	cmpl %ebx, %ecx		# compares ECX to EBX
	jg loop				# if ECX if < EBX jump to loop label !!! The jump instruction have to be under cmpl instruction directly
						# (probably)

end:
	int $0x80
