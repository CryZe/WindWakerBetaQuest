0x802ff7c0:
nop ; Get rid of ClearArena

0x80006458:
bl game_loop
bl 0x80022e74 ; fapGm_Execute__Fv

0x8003e790:
b set_next_stage_hook
