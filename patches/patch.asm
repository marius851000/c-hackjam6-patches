// Replace the "GetMovePower" function with a custom one.
// Since a branch is inserted at the start of the function, the function is practically
// replaced with our own. The "b" instruction doesn't modify the link register, so
// execution will continue after the call to `GetMovePower` once our function returns.

.nds
.include "symbols.asm"

.open "overlay29.bin", overlay29_start
    .org DungeonFrameDrawUIHook
        bl DungeonFrameDrawUIHookImplementation
    
    .org DungeonLoadTextureHook
        bl DungeonLoadTextureImplementation
    
    .org OnTurnHook
        bl OnTurnImplementation

    .org FloorChangeHook
        bl FloorChangeHookImplementation
    
    .org EnemyDefeatedHook
        bl EnemyDefeatedHookImplementation

    .org PostDunGenHook
        bl PostDunGenHookPost
    
    .org DisplayDungeonTipStart
        b DisplayDungeonTipStartHook
    
    .org DisplayDungeonFloorTipPatch
        // always false
        cmp r0, #2
    
    .org DisplayItemTip
        mov r0, #0
        push lr
        pop pc
.close

.open "overlay11.bin", overlay11_start
    // a function called 
    .org FrameHookStart
        bl CustomFrameCode
    
    .org FrameHook2Start
        bl CustomFrame2Code
.close

// Reset taxi counter at dungeon start
.open "arm9.bin", arm9_start
    .org InitializeDungeonHookGetGameModeHook
        bl InitializeDungeonHookImplementation
    
    /*.org BaseCreateCanvasBorder
        b ReimpCreateCanvasBorder*/
    
    .org PatchForLanguage
        mov r4, #0
.close