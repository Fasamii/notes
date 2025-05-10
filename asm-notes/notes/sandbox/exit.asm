.section .text
.global _start
_start:
	movl $2, %eax 		# moves 2 into EAX register
	movl $4, %ebx		# moves 4 into EBX register
	imull %eax, %ebx	# multiplies (EAX * EBX -> EBX) (EBX is alos used to store exit status for exit syscall)
	movl $1,  %eax 		# sys call number for exit
	int  $0x80			# wake kernel to exit
