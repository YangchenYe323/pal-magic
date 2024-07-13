import json
import sys

path = sys.argv[1]
opath = sys.argv[2]

with open(path, "r") as f:
    data = json.load(f)

obj = data
obj = obj["properties"]
obj = obj["worldSaveData"]
obj = obj["value"]
obj = obj["ItemContainerSaveData"]
obj = obj["value"]

for idx, val in enumerate(obj):
    val = val["value"]
    val = val["Slots"]
    val = val["value"]
    val = val["values"]

    curContainerUpdated = False
    for slot in val:
        if slot["ItemId"]["value"]["StaticId"]["value"] == "None":
            slot["ItemId"]["value"]["StaticId"]["value"] = "PalSphere_Ultimate"
            slot["StackCount"]["value"] = 99
            curContainerUpdated = True
            break
        if slot["ItemId"]["value"]["StaticId"]["value"] == "PalSphere_Ultimate":
            slot["StackCount"]["value"] = 99
            curContainerUpdated = True
            break
    if not curContainerUpdated:
        print(f"Container {idx} not updated as there is no free slot")

with open(opath, "w") as f:
    json.dump(data, f)
