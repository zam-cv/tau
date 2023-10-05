#include <stdexcept>
#include "catch.h"
#include "add.h"

TEST_CASE("example test") {
  SECTION("test: add") {
    int a = 1;
    int b = 2;

    REQUIRE(add(a, b) == 3);
  }
}