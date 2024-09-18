
.align 4
InitializeDungeonHookImplementation:
  push {lr}
  bl ResetTaxiCounter
  bl GetGameMode
  pop {pc}
  .pool

.align 4
DungeonFrameDrawUIHookImplementation:
  add sp,sp,#0x44
  push {lr}
  bl TaxiDrawUI
  pop {pc}
  .pool

.align 4
DungeonLoadTextureImplementation:
  push {lr}
  bl FUN_overlay_29__023361b4
  bl LoadTaxiTexture
  pop {pc}
  .pool

.align 4
OnTurnImplementation:
  push {lr}
  bl DecrementWindCounter
  bl TaxiOnTurn
  pop {pc}
  .pool

.align 4
FloorChangeHookImplementation:
  push {lr}
  bl FUN_overlay_29__0233902c
  bl TaxiOnFloorChange
  pop {pc}
  .pool

.align 4
EnemyDefeatedHookImplementation:
  push {lr}
  bl LogMessageByIdWithPopupCheckUserTarget
  bl TaxiOnEnemyDefeated
  pop {pc}
  .pool

.align 4
PostDunGenHookPost:
  push {lr}
  bl FUN_overlay_29__0233b214
  bl spawnSnorlaxOnStairIfAppropriate
  pop {pc}
  .pool

.align 4
DisplayDungeonTipStartHook:
  #original instruction
  .byte 0x38
  .byte 0x40
  .byte 0x2d
  .byte 0xe9
  b DisplayDungeonTipStart+4
  .pool