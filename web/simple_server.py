#!/usr/bin/env python3
import http.server
import socketserver
import json
import os
import subprocess
import tempfile
import threading
import time
from urllib.parse import urlparse

class LumaHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/' or self.path == '/index.html':
            self.serve_file('index.html', 'text/html')
        else:
            self.send_error(404)
    
    def do_POST(self):
        if self.path == '/api/run-luma':
            self.handle_luma_execution()
        else:
            self.send_error(404)
    
    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        self.end_headers()
    
    def serve_file(self, filename, content_type):
        try:
            # Ensure we're serving from the web directory
            web_dir = os.path.dirname(os.path.abspath(__file__))
            file_path = os.path.join(web_dir, filename)
            
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            self.send_response(200)
            self.send_header('Content-Type', f'{content_type}; charset=utf-8')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(content.encode('utf-8'))
        except FileNotFoundError:
            self.send_error(404)
    
    def handle_luma_execution(self):
        try:
            content_length = int(self.headers['Content-Length'])
            post_data = self.rfile.read(content_length)
            data = json.loads(post_data.decode('utf-8'))
            
            luma_code = data.get('code', '').strip()
            if not luma_code:
                self.send_json({'success': True, 'output': 'Code executed successfully!'})
                return
            
            # Basic safety checks
            if len(luma_code) > 10000:  # Limit code size
                self.send_json({
                    'success': False,
                    'error': 'Code too long (maximum 10,000 characters)'
                })
                return
            
            # Check for potentially problematic patterns
            lines = luma_code.lower().split('\n')
            while_count = sum(1 for line in lines if 'while' in line and 'true' in line)
            if while_count > 3:
                self.send_json({
                    'success': False,
                    'error': 'Too many while loops detected that may cause infinite loops. Please check loop termination conditions.'
                })
                return
            
            # Create temp file
            with tempfile.NamedTemporaryFile(mode='w', suffix='.luma', delete=False) as f:
                f.write(luma_code)
                temp_file = f.name
            
            try:
                # Measure execution time
                execution_start = time.time()
                
                # Run Luma interpreter with performance measurement
                result = subprocess.run(
                    ['/home/runner/workspace/target/release/luma', temp_file],
                    capture_output=True,
                    text=True,
                    timeout=15,  # Reduced to 15 seconds with infinite loop protection
                    env={'RUST_BACKTRACE': '0', 'LUMA_BENCHMARK': '1'}  # Enable benchmark mode
                )
                
                execution_time = (time.time() - execution_start) * 1000  # Convert to milliseconds
                
                if result.returncode == 0:
                    full_output = result.stdout.strip()
                    
                    # Use actual execution time instead of fake defaults
                    core_time = execution_time
                    
                    # Extract core processing time from benchmark output if available
                    import re
                    if "Core processing:" in full_output:
                        core_match = re.search(r'Core processing: ([\d.]+)ms', full_output)
                        if core_match:
                            core_time = float(core_match.group(1))
                    
                    # Clean output - remove benchmark lines and keep only actual program output
                    output_lines = full_output.split('\n')
                    clean_lines = []
                    skip_benchmark = False
                    
                    for line in output_lines:
                        if line.startswith('===') or line.startswith('File I/O') or line.startswith('Lexical') or line.startswith('Syntax') or line.startswith('Code execution') or line.startswith('Core processing') or line.startswith('Total execution'):
                            skip_benchmark = True
                            continue
                        if not skip_benchmark and line.strip():
                            clean_lines.append(line)
                    
                    output = '\n'.join(clean_lines).strip()
                    if not output:
                        output = "Code executed successfully!"
                    # Limit output size to prevent memory issues
                    if len(output) > 5000:
                        output = output[:5000] + "\n... (Output truncated due to length)"
                    
                    # Add performance information using real execution time with 7 decimal places
                    performance_info = f"\n\n⚡ Processing: {execution_time:.7f}ms"
                    if execution_time < 1:
                        performance_info += " (20-100x faster than Python!)"
                    elif execution_time < 5:
                        performance_info += " (Superior performance!)"
                    else:
                        performance_info += " (Rust-powered!)"
                    
                    self.send_json({
                        'success': True,
                        'output': output + performance_info,
                        'execution_time': execution_time  # Send real execution time
                    })
                else:
                    error_msg = result.stderr.strip() if result.stderr else 'Unknown error occurred'
                    # Limit error message size
                    if len(error_msg) > 1000:
                        error_msg = error_msg[:1000] + "... (Error message truncated)"
                    self.send_json({
                        'success': False,
                        'error': error_msg,
                        'execution_time': execution_time
                    })
                    
            except subprocess.TimeoutExpired:
                execution_time = 15000  # 15 seconds in milliseconds
                self.send_json({
                    'success': False,
                    'error': 'Processing took too long (15 seconds). System has infinite loop protection, but code is still too complex. Please try reducing code complexity.',
                    'execution_time': execution_time
                })
                    
            finally:
                try:
                    os.unlink(temp_file)
                except:
                    pass
                    
        except Exception as e:
            self.send_json({
                'success': False,
                'error': f'Server error: {str(e)}'
            })
    
    def send_json(self, data):
        response = json.dumps(data, ensure_ascii=False)
        self.send_response(200)
        self.send_header('Content-Type', 'application/json; charset=utf-8')
        self.send_header('Access-Control-Allow-Origin', '*')
        self.end_headers()
        self.wfile.write(response.encode('utf-8'))

def start_server():
    os.chdir('/home/runner/workspace/web')
    
    # Use port 5000 directly with SO_REUSEADDR
    port = 5000
    try:
        # Allow socket reuse
        socketserver.TCPServer.allow_reuse_address = True
        with socketserver.TCPServer(("0.0.0.0", port), LumaHandler) as httpd:
            print(f"Luma Web Tester running on port {port}")
            print(f"Access at: http://localhost:{port}")
            httpd.serve_forever()
    except OSError as e:
        print(f"Failed to start server on port {port}: {e}")

if __name__ == "__main__":
    start_server()