import socketio

sio = socketio.Server()
app = socketio.WSGIApp(sio)

@sio.event
def connect(sid, environ):
    print('Client connected:', sid)

@sio.event
def message(sid, data):
    print('Message from client:', data)
    sio.send(sid, f'Echo: {data}')

@sio.event
def disconnect(sid):
    print('Client disconnected:', sid)

if __name__ == '__main__':
    import eventlet
    import eventlet.wsgi
    eventlet.wsgi.server(eventlet.listen(('localhost', 3000)), app)
