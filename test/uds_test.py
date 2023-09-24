import socket

# Define the server address and port
HOST = 'localhost'
PORT = 13400

def main():
    # Create a socket object and bind it to the specified address and port
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server_socket:
        server_socket.bind((HOST, PORT))
        server_socket.listen()

        print(f"Server listening on {HOST}:{PORT}")

        # Accept incoming client connections
        client_socket, client_address = server_socket.accept()
        print(f"Accepted connection from {client_address}")

        while True:
            # Receive data from the client
            data = client_socket.recv(1024)

            if not data:
                continue  # Connection closed by the client

            # Print the received data in hexadecimal format
            hex_data = data.hex()
            print(f"Received data in hex: {hex_data}")

            # Send a response (e.g., another hex number) back to the client
            response = b'\x02\xfd\x00\x06\x00\x00\x00\x09\x12\x34\x56\x78\x10\x00\x00\x00\x00'  # Example response in hexadecimal bytes
            client_socket.sendall(response)
            print(f"Sent response in hex: {response.hex()}")

            #response 1001
            data = client_socket.recv(1024)
            print(f"Received data in hex: {hex_data}")
            response = b'\x02\xfd\x80\x01\x00\x00\x00\x06\x56\x78\x12\x34\x50\x01'
            client_socket.sendall(response)
            print(f"Sent response in hex: {response.hex()}")

            #response 22f196
            data = client_socket.recv(1024)
            print(f"Received data in hex: {hex_data}")
            response = b'\x02\xfd\x80\x01\x00\x00\x00\x08\x56\x78\x12\x34\x62\xf1\x86\x02'
            client_socket.sendall(response)
            print(f"Sent response in hex: {response.hex()}")

        # Close the client socket
        client_socket.close()

if __name__ == "__main__":
    main()
