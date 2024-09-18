/* !file overlay29 */
/* ApplyMoveEffect = ?????; unknown */
ApplyMoveEffectHookAddr = 0x023302E4;
ApplyMoveEffectJumpAddr = 0x0233310C;
ApplyItemEffectHookAddr = 0x0231C434;
ApplyItemEffectJumpAddr = 0x0231D574;

ApplyMoveEffectHookAddrExtracted = 0x023302F0;
ApplyMoveEffectHookAddrExtractedOnSucess = 0x0233310C;

/* Add your own symbols here... */

/* !file arm9 */
ChangeGlobalBorderColor = 0x02027D74;

ENGINE_DISPLAY_INFO = 0x022a40ec;

DISPCNT_A = 0x04000000;
DISPCNT_B = 0x04001000;

BG3CNT_B = 0x0400100E;
BG3CNT_A = 0x0400000E;
BG1CNT_A = 0x0400000A;

VRAMCNT_A = 0x04000240;
VRAMCNT_C = 0x04000242;
VRAMCNT_D = 0x04000243;
DISPCAPCNT = 0x4000064;


InitializeDungeonHookGetGameModeHook = 0x0204eda0;
ProcessWTEWrapper = 0x0201e1c8;
DoSomethingOn3dAllocAndClearInput = 0x0201e1d8;

BG1_A_GAME_PRIORITY = 0x022a4110;

BaseCreateCanvasBorder = 0x0201ea88;

PatchForLanguage = 0x0202064c;

/* !file overlay11 */

screen_FadeOut = 0x022ea464;
screen_FadeIn = 0x022ea44c;

# This one is just at the start of the frame
FrameHookStart = 0x022e9c4c;
FrameHookReplaced = 0x02008f88;

# This one is near the end, after textboxes
FrameHook2Start = 0x022e9bf0;
FUN_overlay_11__022ebd30 = 0x022ebd30;

/* !file overlay29 */

DungeonFrameDrawUIHook = 0x02336af4;

DungeonLoadTextureHook = 0x022e1b04;
FUN_overlay_29__023361b4 = 0x023361b4;

OnTurnHook = 0x022ec700;

FloorChangeHook = 0x022dffac; # More accuratly, this one is on floor generation
FUN_overlay_29__0233902c = 0x0233902c;

EnemyDefeatedHook = 0x0230a6c0;

PlayWrapOutAnimation = 0x022e726c;

PostDunGenHook = 0x0233ba3c;
FUN_overlay_29__0233b214 = 0x0233b214;
FUN_overlay_29__02305474 = 0x02305474;

DisplayDungeonTipStart = 0x0234daf0;
DisplayDungeonFloorTipPatch = 0x0234d7bc;
DisplayItemTip = 0x0234d8b4;