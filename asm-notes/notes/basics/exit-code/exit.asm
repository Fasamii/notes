.text

.global _start
_start:
	movl $1,  %eax 		# sys call number for exit
	movl $10, %ebx		# exit status code
	int  $0x80			# wake kernel to exit
