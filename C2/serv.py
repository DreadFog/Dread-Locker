from http.server import BaseHTTPRequestHandler, HTTPServer
import cgi

class MyHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        # get the content type and content length of the request
        content_type = self.headers['content-type']
        content_length = int(self.headers['Content-Length'])
        
        # read the request data
        post_data = self.rfile.read(content_length)
        
        # print the request headers and data
        print("POST request received:")
        print(f"Content-Type: {content_type}")
        print(f"Content-Length: {content_length}")
        print("Headers:")
        for header, value in self.headers.items():
            print(f"{header}: {value}")
        print(f"Request Data: {post_data.decode('utf-8')}")

        # send a response to the client
        self.send_response(200)
        self.send_header('Content-type', 'text/html')
        self.end_headers()
        self.wfile.write(b'POST request received')

if __name__ == '__main__':
    server_address = ('', 8000)
    httpd = HTTPServer(server_address, MyHandler)
    print('Server running on port 8080')
    httpd.serve_forever()
