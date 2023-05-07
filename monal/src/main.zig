const std = @import("std");
const expect = std.testing.expect;
const print = std.debug.print;
const eql = std.mem.eql;
const test_allocator = std.testing.allocator;
const out = std.io.getStdOut();
const in = std.io.getStdIn();

pub fn main() !void {
    try out.writeAll("Welcome to Monal Shell\n");
    try repl();
}

fn repl() !void {
    try out.writeAll("mnl>>");

    var buffer: [100]u8 = undefined;

    const input = (try readLine(in.reader(), &buffer)).?;
    try out.writer().print(
        "Your input is: \"{s}\"\n",
        .{input},
    );
    buffer = undefined;
    try repl();
}

fn readLine(reader: anytype, buffer: []u8) !?[]const u8 {
    var line = (try reader.readUntilDelimiterOrEof(
        buffer,
        '\n',
    )) orelse return null;
    // trim annoying windows-only carriage return character
    if (@import("builtin").os.tag == .windows) {
        return std.mem.trimRight(u8, line, "\r");
    } else {
        return line;
    }
}

pub fn xmain() !void {
    var n: []const u8 = "singh";
    _ = n;
    var name: [:0]const u8 = "puran";
    print("My name:{s}\n", .{name});
    name = "narup";
    print("My name:{s}\n", .{name});

    var found_index: ?usize = null;
    const data = [_]i32{ 1, 2, 3, 4, 5, 6, 7, 8, 12 };
    for (data) |v, i| {
        if (v == 10) found_index = i;
    }
    try expect(found_index == null);
    print("Value:{?d}", .{found_index});

    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    var tok = std.zig.Tokenizer.init("def my_function() {}");
    const next = tok.next();

    const val = tok.buffer[next.loc.start..next.loc.end];
    print("VAL: {s}\n", .{val});

    print("Next token {s}\n", .{@tagName(tok.next().tag)});
    print("Next token {s}\n", .{@tagName(tok.next().tag)});
    print("Next token {s}\n", .{@tagName(tok.next().tag)});
    print("Next token {s}\n", .{@tagName(tok.next().tag)});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}

test "if optional payload capture" {
    const a: ?i32 = 5;
    if (a != null) {
        const value = a.?;
        _ = value;
    }

    var b: ?i32 = 5;
    if (b) |*value| {
        value.* += 1;
    }
    try expect(b.? == 6);
}

const User = struct { name: []u8, age: u16 };

test "json parse" {
    var stream = std.json.TokenStream.init(
        \\{ "name": "Joe", "age": 25 }
    );

    const x = try std.json.parse(
        User,
        &stream,
        .{ .allocator = test_allocator },
    );

    defer std.json.parseFree(
        User,
        x,
        .{ .allocator = test_allocator },
    );

    try expect(eql(u8, x.name, "Joe"));
    try expect(x.age == 25);
}
