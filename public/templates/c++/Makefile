COMPILERS = clang++ g++ c++ cpp gcc
CXX = $(firstword $(shell which $(COMPILERS) 2>/dev/null))

ifeq ($(CXX),)
$(error No C++ compiler found in PATH. Please install one or set it manually.)
endif

CFLAGS = -Iinclude -std=c++17
SRC_DIR = src
BUILD_DIR = build
TEST_DIR = tests
LIB_DIR = lib
INCLUDE_DIR = include
TEST_EXECUTABLE = $(BUILD_DIR)/tests

# Platform-specific settings
ifeq ($(OS),Windows_NT)
    RM = del /Q
    FixPath = $(subst /,\,$1)
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        RM = rm -f
    endif
    ifeq ($(UNAME_S),Darwin)
        RM = rm -f
    endif
    FixPath = $1
endif

main: $(BUILD_DIR)/main

$(BUILD_DIR)/main: $(BUILD_DIR)/main.o
	$(CXX) $(CFLAGS) $^ -o $@

$(BUILD_DIR)/main.o: $(SRC_DIR)/main.cpp
	$(CXX) $(CFLAGS) -c $< -o $@

TEST_SOURCES = $(wildcard $(TEST_DIR)/*.cpp)
TEST_OBJECTS = $(patsubst $(TEST_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(TEST_SOURCES))

tests: $(TEST_EXECUTABLE)

$(TEST_EXECUTABLE): $(TEST_OBJECTS)
	$(CXX) $(CFLAGS) $^ -o $@

$(BUILD_DIR)/%.o: $(TEST_DIR)/%.cpp
	$(CXX) $(CFLAGS) -c $< -o $@

clean:
	$(RM) $(call FixPath,$(BUILD_DIR)/main $(TEST_EXECUTABLE) $(BUILD_DIR)/*.o)