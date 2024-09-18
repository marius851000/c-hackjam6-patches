#include <pmdsky.h>
#include <cot.h>

volatile extern int32_t DISPCNT_A;
volatile extern int8_t VRAMCNT_A;
volatile extern int16_t BG3CNT_A;


bool CUSTOM_VRAM_BOTTOM_MODE_ENABLED = 0;

uint8_t VRAMCNT_A_BAK;
uint32_t DISPCNT_A_RELEVANT_BITS_BAK;
uint16_t BG3CNT_A_BAK;

struct EngineDisplayInfo {
    bool disable_bg0;
    bool disable_bg1;
    bool disable_bg2;
    bool disable_bg3;
    bool disable_obj;
};

extern struct EngineDisplayInfo ENGINE_DISPLAY_INFO[2];

struct EngineDisplayInfo ENGINE_DISPLAY_INFO_BAK_A;

void configureBottomScreenDisplayToBitmap() {
    if (CUSTOM_VRAM_BOTTOM_MODE_ENABLED != 0) {
        return;
    }

    VRAMCNT_A_BAK = VRAMCNT_A;


    int32_t dispcnt_A_temp = DISPCNT_A;
    DISPCNT_A_RELEVANT_BITS_BAK = dispcnt_A_temp & 0x00001F03;
    dispcnt_A_temp = (dispcnt_A_temp & 0xFFFFFFF8) | 3; // BG mode (Text/Text/Extended)
    DISPCNT_A = dispcnt_A_temp;

    BG3CNT_A_BAK = BG3CNT_A;
    // 0000_00bb
    BG3CNT_A = 0x4084;

    VRAMCNT_A = 0x81;

    memcpy(&ENGINE_DISPLAY_INFO_BAK_A, &ENGINE_DISPLAY_INFO[0], sizeof(struct EngineDisplayInfo));
    ENGINE_DISPLAY_INFO[0].disable_bg0 = 1;
    ENGINE_DISPLAY_INFO[0].disable_bg1 = 1;
    ENGINE_DISPLAY_INFO[0].disable_bg2 = 1;
    ENGINE_DISPLAY_INFO[0].disable_bg3 = 0;
    ENGINE_DISPLAY_INFO[0].disable_obj = 0;


    //init_drawing("CUSTOM/DRAWING/test.prp");

    CUSTOM_VRAM_BOTTOM_MODE_ENABLED = 1;

    COT_LOG(COT_LOG_CAT_SPECIAL_PROCESS, "bottom VRAM configured");
}

void displayImageOnBottomScreen(const char * filepath) {
    configureBottomScreenDisplayToBitmap();
    

    VRAMCNT_A = 0x80;

    struct file_stream file;
    DataTransferInit();
    FileOpen(&file, filepath);
    FileRead(&file, ((void *) 0x6800000), 0x1FFFF);
    DataTransferStop();

    VRAMCNT_A = 0x81;
}

void bottomScreenReturnToNormal() {
    if (CUSTOM_VRAM_BOTTOM_MODE_ENABLED == 0) {
        return;
    }

    VRAMCNT_A = 0x80;
    for (int addr = 0x6800000; addr < 0x681FFFF; addr+=4) {
        *((int32_t *) addr) = 0x00000000;
    }

    VRAMCNT_A = VRAMCNT_A_BAK;
    DISPCNT_A = (DISPCNT_A & 0xFFFFFFF8) | DISPCNT_A_RELEVANT_BITS_BAK;
    BG3CNT_A = BG3CNT_A_BAK;
    memcpy(&ENGINE_DISPLAY_INFO[0], &ENGINE_DISPLAY_INFO_BAK_A, sizeof(struct EngineDisplayInfo));

    CUSTOM_VRAM_BOTTOM_MODE_ENABLED = 0;

    COT_LOG(COT_LOG_CAT_SPECIAL_PROCESS, "bottom VRAM returned to normal");
}