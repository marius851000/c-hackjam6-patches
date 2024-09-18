#include <pmdsky.h>
#include <cot.h>

enum ScreenTransitionType {
    SCREEN_TRANSITION_NONE = 0,
    // Other pars of the code assume these value are in this exact order
    SCREEN_TRANSITION_ABSTRACT_FADE_OUT_RTL = 1,
    SCREEN_TRANSITION_ABSTRACT_FADE_OUT_LTR = 2,
    SCREEN_TRANSITION_ABSTRACT_FADE_IN_RTL = 3,
    SCREEN_TRANSITION_ABSTRACT_FADE_IN_LTR = 4

};

void startScreenTransition(enum ScreenTransitionType transition_type, uint16_t duration);
void applyTransition();
void applyTransitionAbstract(enum ScreenTransitionType transition_type, uint16_t progress);
uint16_t fp10_multiply(uint16_t in1, uint16_t in2);
uint16_t fp10_Square(uint16_t in1);
uint16_t fp10_Cube(uint16_t in1);
uint16_t fp10_easeOutCubic(uint16_t input);