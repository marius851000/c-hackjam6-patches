print("data reading started")

while true do
    print("loop");
    sprite_alloc_data_ptr = memory.readlong(0x020b052c)
    alloc_3d_stuff_ptr = memory.readlong(sprite_alloc_data_ptr + 0xbc)

    

    some_counter = memory.readshort(alloc_3d_stuff_ptr + 0xC)

    some_array_ptr = memory.readlong(alloc_3d_stuff_ptr + 0x0)

    print(alloc_3d_stuff_ptr)

    print(some_counter)
    
    emu.frameadvance();
end

