const std = @import("std");
const expect = std.testing.expect;

pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Hello World\n", .{});

    try bw.flush();
}

test "simple test" {
    try expect(add(1, 2) == 3);
}
