DTB_PATH := $(wildcard tools/platform_dyn/dtb/$(DTB))

# 检查文件是否存在并打印路径
ifeq ($(strip $(DTB_PATH)),)
$(error DTB file $(DTB) not found in dtb folder)
else
$(info DTB file found at $(DTB_PATH))
endif

override FEATURES := $(shell echo $(FEATURES) | tr ',' ' ')
# 增加 axhal/plat_dyn 到 FEATURES 变量
override FEATURES += axhal/plat_dyn

override FEATURES := $(strip $(FEATURES))

# 打印更新后的 FEATURES 变量（可选）
$(info Updated FEATURES: $(FEATURES))