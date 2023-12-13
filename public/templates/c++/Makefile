COMPILERS = clang++ g++ c++ cpp gcc
CXX = $(firstword $(shell which $(COMPILERS) 2>/dev/null))

ifeq ($(CXX),)
$(error No C++ compiler found in PATH. Please install one or set it manually.)
endif

CFLAGS = -Iinclude -std=c++17
DEBUG = -Wall -g -O0
SRC_DIR = src
BUILD_DIR = build
TEST_DIR = tests
LIB_DIR = lib
INCLUDE_DIR = include
TEST_EXECUTABLE = $(BUILD_DIR)/tests

rwildcard = $(wildcard $1$2) $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2))

SRC_SOURCES := $(call rwildcard,$(SRC_DIR)/,*.cpp)
TEST_SOURCES := $(call rwildcard,$(TEST_DIR)/,*.cpp)

SRC_OBJECTS = $(patsubst $(SRC_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(SRC_SOURCES))
TEST_OBJECTS = $(patsubst $(TEST_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(TEST_SOURCES))

main: $(BUILD_DIR)/main

$(BUILD_DIR)/main: $(SRC_OBJECTS)
	$(CXX) $(CFLAGS) $^ -o $@ 

$(BUILD_DIR)/%.o: $(SRC_DIR)/%.cpp
	@$(call make-dir)
	$(CXX) $(CFLAGS) -c $< -o $@

main-debug: $(BUILD_DIR)/main-debug

$(BUILD_DIR)/main-debug: $(SRC_OBJECTS)
	$(CXX) $(CFLAGS) $(DEBUG) $^ -o $@

tests: $(TEST_EXECUTABLE)

$(TEST_EXECUTABLE): $(TEST_OBJECTS) $(filter-out $(BUILD_DIR)/main.o,$(SRC_OBJECTS))
	$(CXX) $(CFLAGS) $^ -o $@

$(BUILD_DIR)/%.o: $(TEST_DIR)/%.cpp
	@$(call make-dir)
	$(CXX) $(CFLAGS) -c $< -o $@

clean:
	$(RM) $(BUILD_DIR)/*

define make-dir
	@dir=$(dir $@); \
	if [ ! -d "$$dir" ]; then \
		$(MKDIR); \
	fi
endef

ifeq ($(OS),Windows_NT)
    MKDIR = if not exist "$(dir $@)" mkdir "$(dir $@)"
    RM = rmdir /s /q
else
    MKDIR = mkdir -p $(dir $@)
    RM = rm -rf
endif