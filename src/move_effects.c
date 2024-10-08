#include <pmdsky.h>
#include <cot.h>
#include "taxi.h"

// Called when using moves. Should return true if a custom effect was applied.
// This function is only called if the move doesn't fail due to a missing target
bool CustomApplyMoveEffect(
    move_effect_input *data, struct entity *user, struct entity *target, struct move *move)
{
  switch (data->move_id)
  {
  case 365:
    taxiPauseForFloor();
    struct entity *leader = GetLeader();
    if (leader != NULL) {
      LogMessage(leader, "The taxi is stuck in traffic!", true);
    }
    return true;
  default:
    return false;
  }
}
