section .text
global _switch

_switch:
  ;; f
  xor r10, r10
  ;; i
  mov r11, 3
  ;; g
  xor r12, r12
  ;; h
  xor r13, r13
  ;; 
  mov bx, 4
  ;; k
  xor r14, r14
  ;; j
  mov r15, 1


  cmp r14, 0
  je case1
  cmp r14, 1
  je case2
  cmp r14, 2
  je case3
  cmp r14, 3
  je case4
  
  case1:
  mov r10, r11
  sub r10, r15 
  mov rax, r10
  ret
  case2:
  mov r10, r12
  sub r10, r13
  mov rax, r10
  ret
  case3:
  mov r10, r12
  add r10, r13
  mov rax, r10
  ret
  case4:
  mov r10, r11
  add r10, r15
  mov rax, r10
  ret