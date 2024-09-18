#include "snorlax.h"

void FUN_overlay_29__02305474(struct entity* ent, int unk2);

uint16_t SNORLAX_LEVEL = 13;
enum dungeon_id SNORLAX_DUNGEON = (enum dungeon_id) 6;
uint16_t SNORLAX_FLOOR = 9;

void setSnorlaxLevel(uint16_t level) {
    SNORLAX_LEVEL = level;
}

void setSnorlaxDungeon(int dunid) {
    SNORLAX_DUNGEON = (enum dungeon_id) dunid;
}

void setSnorlaxFloor(int floor) {
    SNORLAX_FLOOR = floor;
}

void spawnSnorlaxOnStairIfAppropriate() {
    if (DUNGEON_PTR->id.val == SNORLAX_DUNGEON && DUNGEON_PTR->floor == SNORLAX_FLOOR) {
        struct position* stair_pos = &(DUNGEON_PTR->gen_info).stairs_pos;

        //struct tile* stair_tile = GetTileSafe(stair_pos->x, stair_pos->y);
        
        struct spawned_monster_data spawn_data;
        spawn_data.monster_id.val = (enum monster_id) 143;
        spawn_data.behavior.val = BEHAVIOR_NORMAL_ENEMY_0x0;
        spawn_data.field_0x3 = 0;
        spawn_data.field_0x4 = 0;
        spawn_data.field_0x5 = 0;
        spawn_data.field_0x6 = 0;
        spawn_data.field_0x7 = 0;
        spawn_data.level = SNORLAX_LEVEL; 
        spawn_data.pos = *stair_pos;
        spawn_data.cannot_be_asleep = false;
        spawn_data.field_0xf = 0;


        struct entity* spawned_entity = SpawnMonster(&spawn_data, false);
        if (spawned_entity != NULL) {
            InflictSleepStatusSingle(spawned_entity, 0x7f);
            FUN_overlay_29__02305474(spawned_entity, 8);
        }
    }
}