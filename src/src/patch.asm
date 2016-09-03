0x802ff7c0:
nop ; Get rid of ClearArena

0x80006458:
bl game_loop
bl 0x80022e74 ; fapGm_Execute__Fv

0x8003e790:
b set_next_stage_hook

; Skip Intro
0x80230530: ; dScnOpen_Execute__FP10dScnOpen_c
b 0x8023047c ; changeGameScene__10dScnOpen_cFv

; 0x800A0C38:
; bl ground_cross_hook

; 0x8000645c:
; bl game_loop

; 0x8009fb20:
; nop
; 0x8009fb38:
; nop
; 0x8009e5f0:
; nop
; 0x8009e608:
; nop
; 0x8009e19c:
; nop
; 0x8009e1b4:
; nop
; 0x802445d8:
; nop
; 0x802445f0:
; nop
