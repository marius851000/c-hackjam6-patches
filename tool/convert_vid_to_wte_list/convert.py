from skytemple_files.graphics.wte.handler import WteHandler
from skytemple_files.graphics.wte.model import WteImageType
import sys
from PIL import Image


SOURCE_PATH = "/home/marius/experiment_cot/c-of-time/vid/frames"

for i in range(0, 2000, 10):
    img = Image.open(input_path)


imgtype = None
if int(len(img.getpalette()) / 3) <= 4:
    imgtype = WteImageType.COLOR_2BPP
else:
    imgtype = WteImageType.COLOR_4BPP

wte = WteHandler.new(img, imgtype, False)

bytes = WteHandler.serialize(wte)

f = open(output_path, "wb")
f.write(bytes)
f.close()

print("wte conversion done")