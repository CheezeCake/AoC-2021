const std = @import("std");
const Allocator = std.mem.Allocator;

const Number = struct {
    allocator: Allocator,

    value: ?u32,
    left: ?*Number,
    right: ?*Number,
    parent: ?*Number,

    pub fn create(allocator: Allocator) !*Number {
        const n = try allocator.create(Number);
        n.* = .{ .allocator = allocator, .value = null, .left = null, .right = null, .parent = null };
        return n;
    }

    pub fn destroy(self: *Number) void {
        if (self.left) |left| {
            left.destroy();
        }
        if (self.right) |right| {
            right.destroy();
        }

        self.allocator.destroy(self);
    }

    pub fn clone(self: *const Number, allocator: Allocator) anyerror!*Number {
        const cloned = try Number.create(allocator);
        errdefer cloned.destroy();

        if (self.value) |value| {
            cloned.value = value;
        } else {
            cloned.left = try self.left.?.clone(allocator);
            errdefer cloned.left.?.destroy();
            cloned.left.?.parent = cloned;

            cloned.right = try self.right.?.clone(allocator);
            cloned.right.?.parent = cloned;
        }

        return cloned;
    }

    pub fn add(self: *const Number, other: *const Number, allocator: Allocator) !*Number {
        const result = try Number.create(allocator);
        errdefer {
            result.left = null;
            result.right = null;
            result.destroy();
        }
        const a = try self.clone(allocator);
        errdefer a.destroy();
        const b = try other.clone(allocator);
        errdefer b.destroy();

        result.left = a;
        a.parent = result;

        result.right = b;
        b.parent = result;

        while (try result.reduce(allocator)) {}

        return result;
    }

    pub fn reduce(self: *Number, allocator: Allocator) !bool {
        return self.tryExplode(1) or try self.trySplit(allocator);
    }

    fn predecessor(self: *Number) ?*Number {
        var x: ?*Number = self;
        while (x.?.parent) |parent| {
            if (x.? == parent.left.?) {
                x = parent;
            } else {
                break;
            }
        }

        if (x.?.parent) |parent| {
            x = parent.left;
            while (x.?.right) |right| {
                x = right;
            }
            return x;
        }

        return null;
    }

    fn successor(self: *Number) ?*Number {
        var x: ?*Number = self;
        while (x.?.parent) |parent| {
            if (x.? == parent.right.?) {
                x = parent;
            } else {
                break;
            }
        }

        if (x.?.parent) |parent| {
            x = parent.right;
            while (x.?.left) |left| {
                x = left;
            }
            return x;
        }

        return null;
    }

    fn tryExplode(self: *Number, level: usize) bool {
        if (self.value != null) {
            return false;
        }

        const left = self.left.?;
        const right = self.right.?;

        if (level <= 4) {
            return left.tryExplode(level + 1) or right.tryExplode(level + 1);
        }

        const pred = self.predecessor();
        if (pred) |p| {
            p.value.? += left.value.?;
        }
        const suc = self.successor();
        if (suc) |s| {
            s.value.? += right.value.?;
        }
        self.value = 0;
        self.left = null;
        self.right = null;

        left.destroy();
        right.destroy();

        return true;
    }

    pub fn trySplit(self: *Number, allocator: Allocator) anyerror!bool {
        if (self.value) |value| {
            if (value >= 10) {
                const left = try Number.create(allocator);
                errdefer left.destroy();
                left.value = value / 2;
                left.parent = self;

                const right = try Number.create(allocator);
                right.value = (value + (value % 2)) / 2;
                right.parent = self;

                self.value = null;
                self.left = left;
                self.right = right;

                return true;
            }

            return false;
        }

        return (try self.left.?.trySplit(allocator)) or (try self.right.?.trySplit(allocator));
    }

    fn magnitude(self: *const Number) u32 {
        return self.value orelse 3 * magnitude(self.left.?) + 2 * magnitude(self.right.?);
    }

    fn print(self: *const Number) void {
        if (self.value) |value| {
            std.debug.print("{d}", .{value});
        } else {
            std.debug.print("[", .{});
            self.left.?.print();
            std.debug.print(",", .{});
            self.right.?.print();
            std.debug.print("]", .{});
        }
    }
};

const ParseError = error{ UnexpectedCharacter, ExpectedCharacter };

fn parseNumber(s: []const u8, allocator: Allocator) !*Number {
    var stack = std.ArrayList(*Number).init(allocator);
    defer stack.deinit();
    errdefer {
        while (stack.popOrNull()) |number| {
            number.destroy();
        }
    }

    var i: usize = 0;

    while (i < s.len) : (i += 1) {
        if (s[i] == '[') {
            const number = try Number.create(allocator);
            errdefer number.destroy();

            try stack.append(number);
        } else if (s[i] == ']') {
            const right = stack.pop();
            var parent = stack.pop();

            parent.right = right;
            right.parent = parent;
            try stack.append(parent);
        } else if (s[i] == ',') {
            const left = stack.pop();
            var parent = stack.pop();

            parent.left = left;
            left.parent = parent;
            try stack.append(parent);
        } else if (std.ascii.isDigit(s[i])) {
            var n = s[i] - '0';
            while (i + 1 < s.len and std.ascii.isDigit(s[i + 1])) : (i += 1) {
                n = n * 10 + s[i + 1] - '0';
            }

            var number = try Number.create(allocator);
            errdefer number.destroy();

            number.value = n;
            try stack.append(number);
        } else {
            return ParseError.UnexpectedCharacter;
        }
    }

    if (stack.items.len != 1) {
        return ParseError.ExpectedCharacter;
    }

    return stack.pop();
}

pub fn main() !void {
    const stdin = std.io.getStdIn();
    const stdout = std.io.getStdOut();

    // var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    // defer _ = gpa.deinit();
    // const allocator = gpa.allocator();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    var numbers = std.ArrayList(*Number).init(allocator);
    defer {
        while (numbers.popOrNull()) |number| {
            number.destroy();
        }
        numbers.deinit();
    }
    var result: ?*Number = null;
    defer {
        if (result) |res| {
            res.destroy();
        }
    }

    var buffer: [100]u8 = undefined;
    while (try stdin.reader().readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        var number = try parseNumber(line, allocator);

        try numbers.append(number);

        if (result) |res| {
            result = try res.add(number, allocator);
            res.destroy();
        } else {
            result = try number.clone(allocator);
        }
    }

    try stdout.writer().print("part 1: {d}\n", .{result.?.magnitude()});

    var max_magnitude: u32 = 0;
    for (numbers.items) |n1, i| {
        for (numbers.items) |n2, j| {
            if (i != j) {
                const res = try n1.add(n2, allocator);
                defer res.destroy();

                max_magnitude = std.math.max(max_magnitude, res.magnitude());
            }
        }
    }

    try stdout.writer().print("part 2: {d}\n", .{max_magnitude});
}
