#include "draw_helper.h"

struct render_3d_texture* draw_3d_texture_with_good_default(int x, int y, int texture_offset, int tex_s_size, int tex_t_size, int tex_format, int palette_high, int palette_low) {
    struct render_3d_texture* texture = NewRender3dTexture();
    texture->texture_vram_offset = texture_offset;
    texture->palette_base_addr = palette_high * 0x200 + palette_low * 2;
    texture->scale.x = 0x1000;
    texture->scale.y = 0x1000;
    texture->texture_size.x = 8 << tex_s_size;
    texture->texture_size.y = 8 << tex_t_size;

    texture->color.r = 0x1F;
    texture->color.g = 0x1F;
    texture->color.b = 0x1F;
    texture->alpha = 0x1F;


    texture->texture_params.texture_s_size = tex_s_size;
    texture->texture_params.texture_t_size = tex_t_size;
    texture->texture_params.texture_format = tex_format;

    texture->translation.x = x;
    texture->translation.y = y;

    return texture;
}