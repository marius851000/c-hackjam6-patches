import xml.etree.ElementTree as ET

tree = ET.parse('test.svg')
root = tree.getroot()

commands = []

for child in root.iter():
    if child.tag.endswith("path"):
        x = 0
        y = 0
        last_valid_x = 0
        last_valid_y = 0

        state = "init"
        for section in child.attrib["d"].split(" "):
            if section == "m":
                state = "m"
                continue
            elif section == "c":
                state = "c"
                continue
            elif section == "z":
                state = "z"
                continue
            elif section == "C":
                state = "C"
                continue

            command_to_add = None
            if state == "m" or state == "C":
                x = float(section.split(",")[0])
                y = float(section.split(",")[1])
                command_to_add = {
                    "command": "pen_down",
                    "x": int(x),
                    "y": int(y)
                }
            elif state == "c":
                x += float(section.split(",")[0])
                y += float(section.split(",")[1])
                command_to_add = {
                    "command": "pen_down",
                    "x": int(x),
                    "y": int(y)
                }
            
            if command_to_add != None:
                if command_to_add["x"] > 254 or command_to_add["x"] < 2 or command_to_add["y"] > 254 or command_to_add["y"] < 2:
                    pass
                else:
                    commands.append(command_to_add)
                    last_valid_x = x
                    last_valid_y = y
            
        commands.append({
            "command": "pen_up",
            "x": last_valid_x,
            "y": last_valid_y
        })


test_result = "struct DrawingInfo test_drawing_info[" + str(len(commands) + 1) + " ] = {\n"
for command in commands:
    test_result += "    {\n"
    if command["command"] == "pen_down":
        test_result += "        DRAWING_COMMAND_PEN_DOWN,\n"
    elif command["command"] == "pen_up":
        test_result += "        DRAWING_COMMAND_PEN_UP,\n"
    else:
        raise
    test_result += "        {}, {}\n".format(command["x"], command["y"])
    test_result += "    },\n"
test_result += "    { DRAWING_COMMAND_END, 0, 0 }\n"
test_result += "};"

print(test_result)
            