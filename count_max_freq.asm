default rel 

%define FREQUENCY_SIZE 128

section .data
  ;; how this works is that for example in the position 65 we will store how many times the 
  ;; character ascii 65 is being repeated.
  ;; dd => 32 bits
  repetition: times FREQUENCY_SIZE dd 0x0 ;; 128 positions of 4 bytes.


section .text
global _count_max_freq

_count_max_freq:  
  ;; rdi => char* (string)
  ;; rsi => char*

  xor r15, r15
  ;; Reset array to 0 (Maybe there is another call to this function)
  @loop:
      mov r14, repetition
      mov dword[r14+r15], 0
      inc r15
      cmp r15, FREQUENCY_SIZE
      jle @loop
 
 ;; indexing
  xor r11, r11
  xor r12, r12
  xor r13, r13
  xor r14, r14
  xor r15, r15
  
  ;; rax will store the maximum and will be used for 8 bit moves
  xor rax, rax
  start:
    ;; get memory location (string + index) and store the byte 
    movzx r14, byte [rdi+r11] ;; movzx will extend the byte to a 8 byte number with 0s
    cmp r14, 0 ;; check if it is end of string
    je end ;; jump to the end if it is
  
  add_to_freq:
    ;; get memory location of repetitions array
    mov r15, repetition
    ;; store the frequency
    movsxd r12, dword[r15+r14]
    ;; update the frequency
    inc r12
    ;; store the frequency
    mov [r15+r14], r12
    ;; compare the new frequency
    cmp rax, r12
    ;; jump to dontupdatemax so we dont update the maximum registers values if
    ;; it isnt bigger than the current max
    jg dont_update_max
    ;; mov temporarily r14 (character value) to rax so we can use al 8 bit register
    mov rax, r14
    ;; store 8 bit register (character) to where the char* is
    mov [rsi], al
    ;; take the r12 frequency again into rax because rax is the return value of the frequency
    mov rax, r12
  dont_update_max:
    ;; increase index
    inc r11
    ;; go to the start again
    jmp start
  
  end:
    ret

