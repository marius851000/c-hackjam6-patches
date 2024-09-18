#include <pmdsky.h>
#include <cot.h>
#include "top_screen_management.h"
#include "bottom_screen_management.h"
#include "screen_transition.h"
#include "taxi.h"
#include "snorlax.h"

char temppath[30];
//TODO: check that is work with fade

bool CustomScriptSpecialProcessCall(undefined4* unknown, uint32_t special_process_id, short arg1, short arg2, int* return_val) {
  switch (special_process_id) {

    // 110, custom transition command
    case 110:
      COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "custom transition id %d duration %d", arg1, arg2);
      startScreenTransition(arg1, arg2);
      return true;

    // 111, set number of turn for increasing taxi counter by 1
    case 111:
      COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "taxi increase set to each %d turn", arg1);
      setTaxiIncreaseTurn(arg1);
      return true;
    case 112:
      COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "set enemy defeated point to -%d", arg1);
      setSubPerEnemyDefeated(arg1);
      return true;
    case 113:
      COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "set floor change point to -%d", arg1);
      setSubPerFloorChange(arg1);
      return true;
    
    // 114: set tweakable
    case 114:
      COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "set tweakable %d to %d", arg1, arg2);
      switch (arg1) {
        case 1:
          setSnorlaxLevel(arg2);
          break;
        case 2:
          setSnorlaxDungeon(arg2);
          break;
        case 3:
          setSnorlaxFloor(arg2);
          break;
        default:
          COT_LOG(COT_LOG_CAT_SPECIAL_PROCESS, "unknown tweakable");
      }
      return true;
    
    // 120, display an image on top screen, initializing it if needed
    // The script should make the top screen background would not have changed
    case 120:
      sprintf(temppath, "CUSTOM/SCREEN/%04d.raw", arg1);
      COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "loading top screen raw %s", temppath);
      displayImageOnTopScreen(temppath);
      return true;
    // 121, return top screen to what it was before it was set by 120 or 121
    case 121:
      topScreenReturnToNormal();
      return true;
    // 122, top screen drawing mode
    case 122:
      sprintf(temppath, "CUSTOM/DRAWING/%04d.prp", arg1);
      COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "loading prp %s", temppath);
      initDrawingOnTopScreen(temppath);
      return true;
    // 123, bottom screen display raw. Also need to be restaured with 124
    // Not yet properly tested
    case 123:
      sprintf(temppath, "CUSTOM/SCREEN/%04d.raw", arg1);
      COT_LOGFMT(COT_LOG_CAT_SPECIAL_PROCESS, "loading bottom screen raw %s", temppath);
      displayImageOnBottomScreen(temppath);
      return true;
    // 124, restore bottom screen
    case 124:
      bottomScreenReturnToNormal();
      return true;
    default:
      return false;
  }
}
