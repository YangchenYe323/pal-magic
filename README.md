# pal-magic

This repo is currently scattered script I wrote to modify the save data of palworld (e.g., create more palspheres for myself). The real logic is implemented in `process.py`, and it operates on the JSON format of palworld save file (see [palworld-save-tools](`https://github.com/cheahjs/palworld-save-tools`)) for how to get that JSON format.

The rust codebase does not work now unfortunately as the JSON schema inference is horribly unreliable.