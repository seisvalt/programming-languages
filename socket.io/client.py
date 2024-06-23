import socketio

sio = socketio.Client()

@sio.event
def connect():
    print('Connected to server')
    sio.send('Hello Server')

@sio.event
def message(data):
    print(f'Message from server: {data}')

@sio.event
def disconnect():
    print('Disconnected from server')

if __name__ == '__main__':
    sio.connect('http://localhost:3000')
    sio.wait()
