import socket
import threading

def receive_messages(sock):
    while True:
        try:
            message = sock.recv(1024).decode('utf-8')
            if message:
                print("\nReceived: ", message)
            else:
                break
        except:
            break

def send_messages(sock):
    while True:
        message = input("\nEnter your message: ")
        sock.send(message.encode('utf-8'))

def main():
    host = '127.0.0.1'
    port = 8080

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.connect((host, port))

    receive_thread = threading.Thread(target=receive_messages, args=(sock,))
    send_thread = threading.Thread(target=send_messages, args=(sock,))

    receive_thread.start()
    send_thread.start()

if __name__ == "__main__":
    main()