#!/usr/bin/env python3
import http.server
import socketserver
import json
import os
import subprocess
import tempfile
import uuid
from urllib.parse import urlparse, parse_qs
from pathlib import Path

class LumaWebHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        # Set the directory to serve files from
        super().__init__(*args, directory=".", **kwargs)
    
    def do_GET(self):
        """Handle GET requests"""
        if self.path == '/' or self.path == '/index.html':
            self.path = '/index.html'
        
        # Serve static files
        return super().do_GET()
    
    def do_POST(self):
        """Handle POST requests"""
        if self.path == '/api/run-luma':
            self.handle_run_luma()
        else:
            self.send_error(404, "API endpoint not found")
    
    def handle_run_luma(self):
        """Handle Luma code execution"""
        try:
            # Read the request body
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data.decode('utf-8'))
            
            luma_code = data.get('code', '').strip()
            if not luma_code:
                self.send_json_response({
                    'success': False,
                    'error': 'No code provided'
                })
                return
            
            # Create a temporary file for the Luma code
            with tempfile.NamedTemporaryFile(mode='w', suffix='.luma', delete=False) as f:
                f.write(luma_code)
                temp_file = f.name
            
            try:
                # Build the Luma interpreter if needed
                build_result = subprocess.run(
                    ['cargo', 'build', '--release'],
                    cwd='..',  # Go up one directory to project root
                    capture_output=True,
                    text=True,
                    timeout=60
                )
                
                if build_result.returncode != 0:
                    self.send_json_response({
                        'success': False,
                        'error': f'Build failed: {build_result.stderr}'
                    })
                    return
                
                # Run the Luma interpreter on the temporary file
                result = subprocess.run(
                    ['../target/release/luma', temp_file],
                    capture_output=True,
                    text=True,
                    timeout=30  # 30 second timeout
                )
                
                if result.returncode == 0:
                    self.send_json_response({
                        'success': True,
                        'output': result.stdout.strip()
                    })
                else:
                    self.send_json_response({
                        'success': False,
                        'error': result.stderr.strip() if result.stderr else 'Unknown error occurred'
                    })
                    
            finally:
                # Clean up the temporary file
                try:
                    os.unlink(temp_file)
                except:
                    pass
                    
        except json.JSONDecodeError:
            self.send_json_response({
                'success': False,
                'error': 'Invalid JSON in request'
            })
        except subprocess.TimeoutExpired:
            self.send_json_response({
                'success': False,
                'error': 'Code execution timed out (30 seconds)'
            })
        except Exception as e:
            self.send_json_response({
                'success': False,
                'error': f'Server error: {str(e)}'
            })
    
    def send_json_response(self, data):
        """Send a JSON response"""
        response = json.dumps(data, ensure_ascii=False, indent=2)
        self.send_response(200)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()
        self.wfile.write(response.encode('utf-8'))
    
    def do_OPTIONS(self):
        """Handle CORS preflight requests"""
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()

def main():
    PORT = 5000
    
    # Change to the web directory
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    
    # Allow port reuse
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("0.0.0.0", PORT), LumaWebHandler) as httpd:
        print(f"🌟 Luma Web Tester Server started at http://0.0.0.0:{PORT}")
        print(f"📂 Serving files from: {os.getcwd()}")
        print(f"🔗 Access the web interface at: http://localhost:{PORT}")
        print("Press Ctrl+C to stop the server")
        
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped")

if __name__ == "__main__":
    main()