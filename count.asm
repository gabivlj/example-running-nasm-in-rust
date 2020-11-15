section .text
global _count_bits_extern

_count_bits_extern:
    ;; rdi char*
    ;; rsi int
    push rbp
    xor rbp, rbp
    xor rax, rax
    xor r10, r10 ;; will have the bit
    xor r11, r11
    start:
      cmp rbp, rsi    
      je end
      movzx r10, byte [rbp + rdi] ;; REALMENTE QUIERO HACER ESTO EN VEZ DE TENER QUE USAR REGISTROS DE 8 BITS
    compare_bits:
      ;; check if the byte is 0
      cmp r10, 0
      ;; if it is 0 go to the next byte
      je next_byte
      ;; mov r10 to r11 to do an and
      mov r11, r10
      and r11, 1 ; stores in r11 if the bit is 1
      shr r10, 1 ; shift right
      cmp r11, 0 ; compare r11 to 0
      je dont_increment ; if it is 0 jump to dont increment
      inc rax ; increment if it is 1
    dont_increment:
      jmp compare_bits ; keep comparing
    next_byte:
      inc rbp
      jmp start
    end:
      pop rbp    
      ret