package main

import (
    "database/sql"
    "fmt"
    "log"
    "github.com/go-redis/redis/v8"
    _ "github.com/go-sql-driver/mysql"
    "context"
)

var ctx = context.Background()

func main() {
    // Conexión a MySQL
    db, err := sql.Open("mysql", "user:password@tcp(localhost:3306)/database")
    if err != nil {
        log.Fatal(err)
    }
    defer db.Close()

    // Consulta MySQL
    rows, err := db.Query("SELECT userId, host, created FROM connections WHERE status = 10 AND created > '2022-04-02'")
    if err != nil {
        log.Fatal(err)
    }
    defer rows.Close()

    // Conexión a Redis
    rdb := redis.NewClient(&redis.Options{
        Addr: "localhost:6379",
    })

    blockHost, err := rdb.SMembers(ctx, "blockHost").Result()
    if err != nil {
        log.Fatal(err)
    }

    // Verificación de resultados
    for rows.Next() {
        var userId int
        var host, created string
        if err := rows.Scan(&userId, &host, &created); err != nil {
            log.Fatal(err)
        }

        if contains(blockHost, host) {
            fmt.Println("Block")
        } else {
            fmt.Println("Accept")
        }
    }
}

func contains(slice []string, item string) bool {
    for _, v := range slice {
        if v == item {
            return true
        }
    }
    return false
}
