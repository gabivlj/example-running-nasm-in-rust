
section .text
global _factorial_extern
_factorial_extern:
    push rbp ;; 
    cmp rdi, 0
    je end       
    push rdi
    dec rdi
    call _factorial_extern    
    pop rdi
    mul rdi
    pop rbp
    ret 
  end:
    mov rax, 1
    pop rbp
    ; xor rax, rax
    ret