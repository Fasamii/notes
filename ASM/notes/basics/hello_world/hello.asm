.global _start            			# Make the entry point visible (tells the assembler _start: symbol should be visible outside of this file)

.section .data						# starts data section
    msg:							# tells in program name for string
		.ascii "Hello, World!\n"  	# String .ascii with newline and null terminator

.section .text
_start:								# this is _start: label it marks entry point for the progam (it works like main() in c)

	movl $4,    %eax				# tells which syscall to perform
	movl $1,    %ebx				# tells which file descriptor to use
	movl $msg,  %ecx				# set memory addres of begining of the String
	movl $15,	%edx				# tells how many chars from beggining address to read
	int $0x80						# interupts program (syscall)

	movl $1,	%eax				# tells which syscall to perform
	movl $0, 	%ebx				# sets exit status to 0
	int $0x80						# interupts program (syscall)
