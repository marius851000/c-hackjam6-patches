/**
Screen transition using 3D engine on bottom screen

(I suspect it’s how Irdkwia did it in SUBS now)
*/

#include "screen_transition.h"

void screen_FadeOut(uint32_t duration);
void screen_FadeIn(uint32_t duration);

enum ScreenTransitionType CURRENT_TRANSITION_TYPE = SCREEN_TRANSITION_NONE;
uint16_t TRANSITION_DURATION = 0;
uint16_t TRANSITION_FRAME = 0;

// we can go up to one quarter of 1 in fp10 (0x00FF)
uint16_t PROGRESS_CHANGE[8] = {
    0x80,
    0x40,
    0xA0,
    0x10,
    0x30,
    0x90,
    0xF0,
    0x0
};

void startScreenTransition(enum ScreenTransitionType transition_type, uint16_t duration) {
    CURRENT_TRANSITION_TYPE = transition_type;
    TRANSITION_DURATION = duration;
    TRANSITION_FRAME = 0;

    if (transition_type == SCREEN_TRANSITION_ABSTRACT_FADE_IN_RTL || transition_type == SCREEN_TRANSITION_ABSTRACT_FADE_IN_LTR) {
        screen_FadeIn(0);
    }

    // randomize a bit this list by swapping values
    // give weird result. Disable it for now.
    
    /*for (int i = 0; i < 1000; i++) {
        uint16_t v1_pos = Rand16Bit() >> 12;
        uint16_t v2_pos = Rand16Bit() >> 12;
        uint16_t v1_bak = PROGRESS_CHANGE[v1_pos];
        PROGRESS_CHANGE[v1_pos] = PROGRESS_CHANGE[v2_pos];
        PROGRESS_CHANGE[v2_pos] = v1_bak;
    }*/
}

// Need to be called on each frame, around the end of the generation (at least after dialogue box, so it can fade over the dialogue box, given it’s order that’s used for the 3D engine)
void applyTransition() {
    if (CURRENT_TRANSITION_TYPE == SCREEN_TRANSITION_NONE) {
        return;
    }
    
    uint16_t progress;
    if (TRANSITION_FRAME == TRANSITION_DURATION) {
        if (CURRENT_TRANSITION_TYPE == SCREEN_TRANSITION_ABSTRACT_FADE_OUT_RTL || CURRENT_TRANSITION_TYPE == SCREEN_TRANSITION_ABSTRACT_FADE_OUT_LTR) {
            screen_FadeOut(0);
        }
        TRANSITION_FRAME = 0;
        CURRENT_TRANSITION_TYPE = SCREEN_TRANSITION_NONE;
        return;
    } else {
        progress = _s32_div_f(TRANSITION_FRAME << 16, TRANSITION_DURATION);
        TRANSITION_FRAME += 1;
    }

    if (CURRENT_TRANSITION_TYPE >= SCREEN_TRANSITION_ABSTRACT_FADE_OUT_RTL && CURRENT_TRANSITION_TYPE <= SCREEN_TRANSITION_ABSTRACT_FADE_IN_LTR) {
        applyTransitionAbstract(CURRENT_TRANSITION_TYPE, progress);
    }
}

void FUN_overlay_11__022ebd30(void);

__attribute__((used)) void CustomFrame2Code() {
    applyTransition();

    FUN_overlay_11__022ebd30();
}

uint16_t fp10_multiply(uint16_t in1, uint16_t in2) {
    return ((uint16_t) in1 * (uint16_t) in2) >> 10;
}

uint16_t fp10_Square(uint16_t in1) {
    return fp10_multiply(in1, in1);
}

uint16_t fp10_Cube(uint16_t in1) {
    return fp10_multiply(fp10_Square(in1), in1);
}

uint16_t fp10_Pow(uint16_t in, int amount) {
    uint16_t r = in;
    for (int i = 1; i < amount; i++) {
        r = fp10_multiply(r, in);
    }
    return r;
}

// https://easings.net/#easeOutCubic
uint16_t fp10_easeOutCubic(uint16_t input) {
    return (1 << 10) - fp10_Cube((1 << 10) - input);
}

// https://easings.net/#easeOutQuint
uint16_t fp10_easeOutQuint(uint16_t input) {
    return (1 << 10) - fp10_Pow((1 << 10) - input, 5);
}

void applyTransitionAbstract(enum ScreenTransitionType transition_type, uint16_t progress) {

    

    progress = progress >> 6; // convert to fp10
    if (transition_type == SCREEN_TRANSITION_ABSTRACT_FADE_IN_RTL || transition_type == SCREEN_TRANSITION_ABSTRACT_FADE_IN_LTR) {
        progress = 0x400 - progress;
    }

    progress = progress - (progress >> 2) - (progress >> 3); // multiply by 1.25

    // we divide the 192 px of height in height 24 px slice
    for (int i = 0; i < 8; i++) {
        uint16_t progress_eased;
        if (progress < PROGRESS_CHANGE[i]) {
            progress_eased = 0;
        } else {
            uint16_t changed_progress_base = progress - PROGRESS_CHANGE[i];
            if (changed_progress_base > 0x400) {
                progress_eased = 0x400;
            } else {
                progress_eased = fp10_easeOutQuint(changed_progress_base);
            }
        }
        
        struct render_3d_rectangle* test_data = NewRender3dRectangle();

        if (transition_type == SCREEN_TRANSITION_ABSTRACT_FADE_OUT_RTL || transition_type == SCREEN_TRANSITION_ABSTRACT_FADE_IN_RTL) {
            test_data->size.x = (progress_eased >> 2) + (progress_eased >> 4); // multiply by 1.25
            test_data->translation.x = 0;
        } else {
            test_data->size.x = (progress_eased >> 2) + (progress_eased >> 4);;
            test_data->translation.x = 256 - ((progress_eased >> 2) + (progress_eased >> 4));
        }
        test_data->size.y = 24;
        test_data->translation.y = i * 24;
        test_data->scale.x = 0x1000;
        test_data->scale.y = 0x1000;
        test_data->alpha = 31;
    }
}