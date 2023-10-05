import { expect, test } from "bun:test";

function add(a: number, b: number): number {
  return a + b;
}

test("add", () => {
  expect(add(1, 2)).toBe(3);
});