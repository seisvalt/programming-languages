package main

import (
    "log"
    "time"

    socketio_client "github.com/googollee/go-socket.io-client"
)

func main() {
    serverUrl := "http://localhost:3000"
    opts := &socketio_client.Options{
        Transport: "websocket",
    }

    client, err := socketio_client.NewClient(serverUrl, opts)
    if err != nil {
        log.Fatal(err)
    }

    client.OnConnect("/", func() {
        log.Println("Connected to server")
        client.Emit("message", "Hello Server")
    })

    client.OnEvent("/", "message", func(msg string) {
        log.Println("Message from server:", msg)
    })

    client.OnDisconnect("/", func() {
        log.Println("Disconnected from server")
    })

    // Keep the client running
    select {}
}
