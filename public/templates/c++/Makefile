CC = g++
CFLAGS = -Iinclude -std=c++17
SRC_DIR = src
BUILD_DIR = build
TEST_DIR = tests
LIB_DIR = lib
INCLUDE_DIR = include
TEST_EXECUTABLE = $(BUILD_DIR)/tests

main: $(BUILD_DIR)/main

$(BUILD_DIR)/main: $(BUILD_DIR)/main.o
	$(CC) $(CFLAGS) $^ -o $@ 

$(BUILD_DIR)/main.o: $(SRC_DIR)/main.cpp
	$(CC) $(CFLAGS) -c $< -o $@

TEST_SOURCES = $(wildcard $(TEST_DIR)/*.cpp)
TEST_OBJECTS = $(patsubst $(TEST_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(TEST_SOURCES))

tests: $(TEST_EXECUTABLE)

$(TEST_EXECUTABLE): $(TEST_OBJECTS)
	$(CC) $(CFLAGS) $^ -o $@

$(BUILD_DIR)/%.o: $(TEST_DIR)/%.cpp
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(BUILD_DIR)/main $(TEST_EXECUTABLE) $(BUILD_DIR)/*.o