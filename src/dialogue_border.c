#include <pmdsky.h>
#include <cot.h>
#include "draw_helper.h"

void ProcessWTEWrapper(struct wte_handle *wte_ref,undefined4 dest_vram_offset,undefined4 param_3, undefined4 param_4);
void DoSomethingOn3dAllocAndClearInput(void **wte_ref);

bool HAS_DATA_BEEN_LOADED = false;

// TODO: double check the data fit that space
// data loaded at 0x1f800, palette 0x1F + 0x80
void LoadFrameDataIfNeeded() {
  // The game’s load frame code is called before the loading of ov36, so load it the first time it is needed (should take less than a frame anyway)
  if (HAS_DATA_BEEN_LOADED == false) {
    struct wte_handle wte_ref;
    LoadWteFromRom(&wte_ref, "/CUSTOM/VRAM/dialside.wte", 0);
    ProcessWTEWrapper(&wte_ref, 0x1f800, 0x1F, 0x80);
    DoSomethingOn3dAllocAndClearInput(&wte_ref.content);
    HAS_DATA_BEEN_LOADED = true;
  }
}

void DrawDialogueElegantBorder(int x_pos, int y_pos, int height, bool flip) {
  int size_top = 0;
  int size_middle = 0;
  int size_bottom = 0;
  int flip_offset = 0;
  if (flip != false) {
    flip_offset = 16;
  }

  int number_part = height / 8;

  if (height > 72) {
    size_top = 40;
    size_bottom = 32;
    size_middle = height - size_top - size_bottom;
  } else {
    size_top = size_bottom = (number_part / 2) * 8;
    if (number_part % 2 == 1) {
      size_top += 8;
    }
  }

  struct render_3d_texture* texture_top = draw_3d_texture_with_good_default(x_pos, y_pos, 0x1f800, 2, 4, 2, 0x1F, 0x80);
  texture_top->texture_size.x = 8;
  texture_top->texture_size.y = size_top;
  texture_top->texture_top_left.x = flip_offset;

  struct render_3d_texture* texture_bottom = draw_3d_texture_with_good_default(x_pos, y_pos + size_top + size_middle, 0x1f800, 2, 4, 2, 0x1F, 0x80);
  texture_bottom->texture_size.x = 8;
  texture_bottom->texture_size.y = size_bottom;
  texture_bottom->texture_top_left.y = 72 - size_bottom;
  texture_bottom->texture_top_left.x = flip_offset;

  if (size_middle >= 0) {
    struct render_3d_texture* texture_middle = draw_3d_texture_with_good_default(x_pos, y_pos + size_top, 0x1f800, 2, 4, 2, 0x1F, 0x80);
    texture_middle->texture_size.x = 8;
    texture_middle->texture_size.y = size_middle;
    texture_middle->texture_top_left.y = 72;
    texture_middle->texture_top_left.x = flip_offset;
  }


}

// canvas filling function with id 6
__attribute__ ((used)) void ReimpCreateCanvasBorder(struct render_3d_element_64 *param) {
  LoadFrameDataIfNeeded();

  DrawDialogueElegantBorder(param->vec[0].x, param->vec[0].y, param->vec[1].y, false);
  DrawDialogueElegantBorder(param->vec[0].x + param->vec[1].x - 8, param->vec[0].y, param->vec[1].y, true);
}