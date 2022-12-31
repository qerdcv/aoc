BUILD_PATH :=
ifeq ($(OS),Windows_NT)
	BUILD_PATH = .\build\main.exe
else
	BUILD_PATH = ./build/main
endif

day%:
	@gcc -o $(BUILD_PATH) day_$*.c
	$(BUILD_PATH)
