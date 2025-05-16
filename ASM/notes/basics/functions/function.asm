.section .text

.global _start
_start:
	pushl $20
	pushl $10
	call .foo
	addl $8, %esp
	movl %eax, %ebx
	movl $1, %eax
	int $0x80

.type foo, @function
.foo:
	pushl %ebp			# push base pointer to stack (4B)
	movl %esp, %ebp 	# move stack pointer to base pointer
	movl 8(%ebp), %eax	# movles second last stack value to EAX register
	addl 12(%ebp), %eax # adds third last stack value and val from EAX 

	movl %ebp, %esp		# restore stack pointer
	popl %ebp			# restore base pointer
	ret

