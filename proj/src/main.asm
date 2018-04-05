bits 16
[org 0x7c00]
initialize:
    mov bx, 0x8000 
    mov ss, bx ; move stack segment to 0x8000
    mov bp, sp ; resets base pointer

    mov ax, 0000h ;set video mode
    int 10h

    mov ax, 0500h ;set page
    int 10h

    mov ax, 0b00h ;set backgroun color to green
    mov bx, 0002h
    int 10h
_main:
    mov ax, 1300h ;display message
    mov bx, 000fh
    mov cx, len
    mov dx, 0h
    mov bp, hello_world
    int 10h
    
    mov bp, sp

hello_world db 'Hello, world!',0xa,0
len $-hello_world