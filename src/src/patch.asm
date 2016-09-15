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

; Hook Stage Loading
0x800411e4:
bl stage_load_hook

; On Boomerang
0x800c0c80:
b on_boomerang

; On Bomb Bag
0x800c0d74:
b on_bombs

; On Bow
0x800c0b20:
b on_bow

; On Deku Leaf
0x800c0e0c:
b on_deku_leaf

; On Hookshot
0x800c0cec:
b on_hookshot

; On Skull Hammer
0x800c0dc8:
b on_skull_hammer

; On Save
0x800533ac:
bl on_save
