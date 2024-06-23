package main

import (
    "log"
    "net/http"

    socketio "github.com/googollee/go-socket.io"
)

func main() {
    server, err := socketio.NewServer(nil)
    if err != nil {
        log.Fatal(err)
    }

    server.OnConnect("/", func(s socketio.Conn) error {
        s.SetContext("")
        log.Println("Connected:", s.ID())
        return nil
    })

    server.OnEvent("/", "message", func(s socketio.Conn, msg string) {
        log.Println("Message from client:", msg)
        s.Emit("message", "Echo: "+msg)
    })

    server.OnDisconnect("/", func(s socketio.Conn, reason string) {
        log.Println("Disconnected:", s.ID())
    })

    http.Handle("/socket.io/", server)
    log.Println("Server listening on port 3000")
    log.Fatal(http.ListenAndServe(":3000", nil))
}
