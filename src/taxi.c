#include "taxi.h"

uint16_t TAXI_COUNTER = 0;
uint16_t TURN_SINCE_LAST_TURN_INCREASE = 0;

const uint16_t TAXI_MAX = 100;
const uint16_t TAXI_TIME_COUNT = 80;
uint16_t INCREASE_TURN_EACH = 20;
int16_t SUB_FLOOR_CHANGE = 10;
int16_t SUB_ENEMY_DEFEATED = 5;
bool TAXI_NO_INCREASE_FOR_FLOOR = false;

volatile extern uint16_t BG1CNT_A;
extern uint16_t BG1_A_GAME_PRIORITY;

void ProcessWTEWrapper(struct wte_handle *wte_ref,undefined4 dest_vram_offset,undefined4 param_3, undefined4 param_4);
void DoSomethingOn3dAllocAndClearInput(void **wte_ref);

void PlayTaxiTimeAnimation();
void PlayTaxiLoseAnimation();

void setTaxiIncreaseTurn(int turn) {
    INCREASE_TURN_EACH = turn;
}

void setSubPerEnemyDefeated(int point) {
    SUB_ENEMY_DEFEATED = point;
}

void setSubPerFloorChange(int point) {
    SUB_FLOOR_CHANGE = point;
}

void taxiPauseForFloor() {
    TAXI_NO_INCREASE_FOR_FLOOR = true;
}

void ResetTaxiCounter() {
    TAXI_COUNTER = 0;
    TURN_SINCE_LAST_TURN_INCREASE = 0;
    TAXI_NO_INCREASE_FOR_FLOOR = false;
}

// taxi time background at 0x11000 (4KiB, 2BPP), pal 0x10
// taxi time foreground at 0x15000 (4KiB, 2BPP), pal 0x11
// big taxi for losing at 0x11000 (8KiB, 4BPP), pal 0x10
// taxi car at 0x19000 (1KiB, 4BPP), pal 0x12
void LoadTaxiTexture() {
    // ResetTaxiCounter was too early to keep the palette

    struct wte_handle wte_ref;
    LoadWteFromRom(&wte_ref, "/CUSTOM/VRAM/pizback.wte", 0);
    ProcessWTEWrapper(&wte_ref, 0x11000, 0x10, 0);
    DoSomethingOn3dAllocAndClearInput(&wte_ref.content);


    LoadWteFromRom(&wte_ref, "/CUSTOM/VRAM/pizfront.wte", 0);
    ProcessWTEWrapper(&wte_ref, 0x15000, 0x11, 0);
    DoSomethingOn3dAllocAndClearInput(&wte_ref.content);

    LoadWteFromRom(&wte_ref, "/CUSTOM/VRAM/pizcar.wte", 0);
    ProcessWTEWrapper(&wte_ref, 0x19000, 0x12, 0);
    DoSomethingOn3dAllocAndClearInput(&wte_ref.content);
}

void ChangeValueRelative(int16_t value) {
    if (value > 0) {
        if (TAXI_NO_INCREASE_FOR_FLOOR == false) {
            TAXI_COUNTER += value;
            if (TAXI_COUNTER > TAXI_MAX) {
                TAXI_COUNTER = TAXI_MAX;
            }
            if (TAXI_COUNTER >= TAXI_MAX) {
                //TODO: Do something
            }
        }
    } else if (value < 0) {
        if ((-value) >= TAXI_COUNTER) {
            TAXI_COUNTER = 0;
        } else {
            TAXI_COUNTER += value;
        }
    }
}
void PlayWrapOutAnimation(struct entity* ent);

void CheckValueDuringturn() {
    if (TAXI_COUNTER >= TAXI_MAX) {
        struct entity *leader = GetLeader();
        if (leader != NULL) {
            PlayTaxiLoseAnimation();
            LogMessage(leader, "You can’t escape the taxi", true);
            HandleFaint(leader, (union damage_source) (enum damage_source_non_move) 0x235, leader);
        }
    } else if (TAXI_COUNTER == TAXI_TIME_COUNT) {
        PlayTaxiTimeAnimation();
    }
}

void TaxiOnTurn() {
    TURN_SINCE_LAST_TURN_INCREASE++;
    if (TURN_SINCE_LAST_TURN_INCREASE >= INCREASE_TURN_EACH) {
        TURN_SINCE_LAST_TURN_INCREASE = 0;
        ChangeValueRelative(+1);
        CheckValueDuringturn();
    }
}

void TaxiOnFloorChange() {
    ChangeValueRelative(-SUB_FLOOR_CHANGE);
    TAXI_NO_INCREASE_FOR_FLOOR = false;
}

void TaxiOnEnemyDefeated() {
    ChangeValueRelative(-SUB_ENEMY_DEFEATED);
}


void DrawTaxi(int offset_x, int frame) {
    struct render_3d_texture* taxi_car = NewRender3dTexture();
    taxi_car->texture_vram_offset = 0x11000;
    taxi_car->palette_base_addr = 0x2000;
    taxi_car->texture_size.x = 100;
    taxi_car->texture_size.y = 100;
    taxi_car->scale.x = 0x1000;
    taxi_car->scale.y = 0x1000;
    
    taxi_car->color.r = 0x1F;
    taxi_car->color.g = 0x1F;
    taxi_car->color.b = 0x1F;
    taxi_car->alpha = 0x1F;

    taxi_car->texture_params.texture_s_size = 6;
    taxi_car->texture_params.texture_t_size = 4;
    taxi_car->texture_params.texture_format = 3;

    taxi_car->texture_top_left.x = frame * 100;
    taxi_car->translation.x = offset_x;
    taxi_car->translation.y = 14;
}

bool SHOULD_DRAW_TAXI_IN_FRAME = false;

const int DRAW_TAXI_PAUSE = 90;

void PlayTaxiLoseAnimation() {
    struct wte_handle wte_ref;

    LoadWteFromRom(&wte_ref, "/CUSTOM/VRAM/piztaxbi.wte", 0);
    ProcessWTEWrapper(&wte_ref, 0x11000, 0x10, 0);
    DoSomethingOn3dAllocAndClearInput(&wte_ref.content);

    int anim_counter = 0;
    int current_frame = 0;

    int car_pos = -100;
    int pause_progress = 0;
    bool is_second_phase = false;

    while (true) {
        anim_counter++;
        if (anim_counter > 23) {
            current_frame++;
            anim_counter = 0;
        }

        if (current_frame >= 3) {
            current_frame = 0;
        }

        if (car_pos <= DRAW_TAXI_PAUSE || is_second_phase != false) {
            car_pos += 2;
        } else if (is_second_phase == false) {
            anim_counter = 0;
            pause_progress += 1;
            current_frame = 0;
        }

        if (car_pos >= 300) {
            break;
        }

        if (pause_progress == 60) {
            struct entity *leader = GetLeader();
            if (leader != NULL) {
                leader->is_visible = false;
            }
            SHOULD_DRAW_TAXI_IN_FRAME = true;
            for (int monster_id = 0; monster_id <= 3; monster_id++) {
                struct entity* monster = (DUNGEON_PTR->entity_table).header.monster_slot_ptrs[monster_id];
                if (EntityIsValid(monster) != false) {
                    if (monster->is_visible != false) {
                        PlayWrapOutAnimation(monster);
                    }
                }
            }
            SHOULD_DRAW_TAXI_IN_FRAME = false;
        } else if (pause_progress >= 120) {
            pause_progress = 0;
            is_second_phase = true;
        }

        DrawTaxi(car_pos, current_frame);

        AdvanceFrame(0);
    }
}

void PlayTaxiTimeAnimation() {
    uint16_t old_bg1_priority = BG1_A_GAME_PRIORITY;
    BG1_A_GAME_PRIORITY = 0x1;
    unsigned int taxi_anim_counter = 0;
    unsigned int taxi_anim_phase = 0;
    unsigned int cycle_count = 0;

    while (true) {
        taxi_anim_counter += 1;

        if (taxi_anim_counter >= 50 && taxi_anim_phase >= 2) {
            break;
        }

        if (taxi_anim_counter >= 8 && cycle_count < 16) {
            taxi_anim_counter = 0;
            taxi_anim_phase += 1;
        }

        if (taxi_anim_phase >= 2 && cycle_count < 16) {
            taxi_anim_phase = 0;
            cycle_count += 1;
            if (cycle_count >= 16) {
                taxi_anim_phase = 2;
            }
        }


        struct render_3d_texture* taxi_front = NewRender3dTexture();
        taxi_front->texture_vram_offset = 0x15000;
        taxi_front->palette_base_addr = 0x2200;
        taxi_front->texture_size.x = 256;
        taxi_front->texture_size.y = 256;
        taxi_front->translation.y = -36;
        taxi_front->scale.x = 0x1000;
        taxi_front->scale.y = 0x1000;
        
        taxi_front->color.r = 0x1F;
        taxi_front->color.g = 0x1F;
        taxi_front->color.b = 0x1F;
        taxi_front->alpha = 0x1F;

        taxi_front->texture_params.texture_s_size = 5;
        taxi_front->texture_params.texture_t_size = 5;
        taxi_front->texture_params.texture_format = 2;


        struct render_3d_texture* taxi_back = NewRender3dTexture();
        taxi_back->texture_vram_offset = 0x11000;
        taxi_back->palette_base_addr = 0x2000;
        taxi_back->texture_size.x = 256;
        taxi_back->texture_size.y = 256;
        taxi_back->translation.y = -36;
        taxi_back->hdr.z_index = 0x1;

        taxi_back->color.r = 0x1F;
        taxi_back->color.g = 0x1F;
        taxi_back->color.b = 0x1F;
        taxi_back->alpha = 0x1F;

        taxi_back->texture_params.texture_s_size = 5;
        taxi_back->texture_params.texture_t_size = 5;
        taxi_back->texture_params.texture_format = 2;

        if (taxi_anim_phase == 0) {
            taxi_back->scale.x = 0x1000;
            taxi_back->scale.y = 0x1000;
        } else if (taxi_anim_phase >= 1) {
            taxi_back->scale.x = 0x0F80;
            taxi_back->scale.y = 0x0F80;
            taxi_back->translation.x += 4;
            taxi_back->translation.y += 4;
            taxi_front->translation.y -= 3;

            if (taxi_anim_phase >= 2) {
                taxi_front->translation.y -= taxi_anim_counter * 4;
                if (taxi_anim_counter <= 0x1F) {
                    taxi_back->alpha = 0x1F - taxi_anim_counter;
                } else {
                    taxi_back->alpha = 0;
                }
            }
        }

        AdvanceFrame(0);
    }

    BG1_A_GAME_PRIORITY = old_bg1_priority;
}

uint8_t TAXI_CAR_ANIM_COUNTER = 0;

void TaxiDrawUI() {
    if (TAXI_NO_INCREASE_FOR_FLOOR == false) {
        TAXI_CAR_ANIM_COUNTER += 1;
    }
    if (TAXI_CAR_ANIM_COUNTER > 0x80) {
        TAXI_CAR_ANIM_COUNTER = 0;
    }

    int taxi_progession = (TAXI_COUNTER * 8) / TAXI_MAX;
    if (taxi_progession > 7) {
        taxi_progession = 7;
    }
    int row = taxi_progession % 4;
    int line = taxi_progession / 4;

    struct render_3d_texture* taxi_car_render = NewRender3dTexture();
    taxi_car_render->texture_vram_offset = 0x19000;
    taxi_car_render->palette_base_addr = 0x2400;
    taxi_car_render->texture_size.x = 32;
    taxi_car_render->texture_size.y = 16;
    taxi_car_render->scale.x = 0x1000;
    taxi_car_render->scale.y = 0x1000;
    taxi_car_render->hdr.z_index = 0xFF;
    
    taxi_car_render->color.r = 0x1F;
    taxi_car_render->color.g = 0x1F;
    taxi_car_render->color.b = 0x1F;
    taxi_car_render->alpha = 0x1F;

    taxi_car_render->texture_params.texture_s_size = 4;
    taxi_car_render->texture_params.texture_t_size = 3;
    taxi_car_render->texture_params.texture_format = 3;

    taxi_car_render->translation.x = 220;
    taxi_car_render->translation.y = 2;

    taxi_car_render->texture_top_left.x = row * 32;
    taxi_car_render->texture_top_left.y = line * 32;

    if (TAXI_CAR_ANIM_COUNTER > 0x40) {
        taxi_car_render->texture_top_left.y += 16;
    }

    DisplayNumberTextureUi(228, 6, TAXI_COUNTER, 0);

    if (SHOULD_DRAW_TAXI_IN_FRAME != false) {
        DrawTaxi(DRAW_TAXI_PAUSE + 2, 0);
    }
}