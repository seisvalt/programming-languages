const std = @import("std");
const mysql = @import("mysql");
const redis = @import("redis");

pub fn main() !void {
    const allocator = std.heap.page_allocator;

    // Conexión a MySQL
    var mysql_conn = try mysql.Connection.connect("user:password@tcp(localhost:3306)/database", allocator);

    defer mysql_conn.close();

    const query = "SELECT userId, host, created FROM connections WHERE status = ? AND created > ?";
    const params = .{10, "2022-04-02"};
    var results = try mysql_conn.query(query, params);

    // Conexión a Redis
    var redis_conn = try redis.Connection.connect("localhost:6379", allocator);

    defer redis_conn.close();

    var block_host: []const u8 = try redis_conn.smembers("blockHost");

    // Verificación de resultados
    while (results.next()) |row| {
        const host = row[1];
        if (block_host.contains(host)) {
            std.debug.print("Block\n", .{});
        } else {
            std.debug.print("Accept\n", .{});
        }
    }
}
