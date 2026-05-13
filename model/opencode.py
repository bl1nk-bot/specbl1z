import modal
import httpx
import json
import os
import uuid
import subprocess
import glob as glob_module
import re
from typing import List, Dict, Any

app = modal.App("kilocode-gateway")

# ─── Image ───────────────────────────────────────────────────────────────────

image = (
    modal.Image.debian_slim(python_version="3.12")
    .apt_install("curl", "git", "openssh-client", "nodejs", "npm", "wget")
    .pip_install("fastapi", "starlette", "httpx")
    .env({"PATH": "/usr/local/bin:${PATH}"})
)

# ─── Tools Implementation ──────────────────────────────────────────────────

def tool_read(path: str) -> dict:
    """อ่านไฟล์"""
    try:
        with open(path, "r") as f:
            return {"success": True, "content": f.read()}
    except Exception as e:
        return {"success": False, "error": str(e)}

def tool_write(path: str, content: str) -> dict:
    """เขียนไฟล์"""
    try:
        os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
        with open(path, "w") as f:
            f.write(content)
        return {"success": True, "message": f"Written {len(content)} bytes to {path}"}
    except Exception as e:
        return {"success": False, "error": str(e)}

def tool_edit(path: str, old_string: str, new_string: str) -> dict:
    """แก้ไขเนื้อหาไฟล์แบบเฉพาะจุด (Replace)"""
    try:
        if not os.path.exists(path):
            return {"success": False, "error": f"File not found: {path}"}
        
        with open(path, "r") as f:
            content = f.read()
            
        if old_string not in content:
            return {"success": False, "error": f"String not found in file: {old_string}"}
            
        new_content = content.replace(old_string, new_string)
        
        with open(path, "w") as f:
            f.write(new_content)
            
        return {"success": True, "message": f"Successfully replaced text in {path}"}
    except Exception as e:
        return {"success": False, "error": str(e)}

def tool_list_directory(path: str = ".", tree: bool = False) -> dict:
    """แสดงรายชื่อไฟล์และโฟลเดอร์"""
    try:
        if not os.path.exists(path):
            return {"success": False, "error": f"Path not found: {path}"}
        
        if not tree:
            items = os.listdir(path)
            return {"success": True, "content": "\n".join(items)}
        
        # Tree mode
        output = []
        def walk(current_path, indent=""):
            try:
                items = sorted(os.listdir(current_path))
                for item in items:
                    full_path = os.path.join(current_path, item)
                    is_dir = os.path.isdir(full_path)
                    suffix = "/" if is_dir else ""
                    output.append(f"{indent}{item}{suffix}")
                    if is_dir:
                        walk(full_path, indent + "  ")
            except PermissionError:
                output.append(f"{indent}(Permission Denied)")

        walk(path)
        return {"success": True, "content": "\n".join(output)}
    except Exception as e:
        return {"success": False, "error": str(e)}

def tool_bash(command: str, timeout: int = 30) -> dict:
    """รัน bash command"""
    try:
        result = subprocess.run(
            command, shell=True, capture_output=True, text=True, timeout=timeout
        )
        return {
            "success": result.returncode == 0,
            "stdout": result.stdout[:5000],
            "stderr": result.stderr[:2000],
            "returncode": result.returncode,
        }
    except subprocess.TimeoutExpired:
        return {"success": False, "error": f"Timeout after {timeout}s"}
    except Exception as e:
        return {"success": False, "error": str(e)}

def tool_grep(pattern: str, path: str = ".", glob_pattern: str = None) -> dict:
    """ค้นหาด้วย regex"""
    try:
        cmd = f"grep -rn '{pattern}' {path}"
        if glob_pattern:
            cmd += f" --include='{glob_pattern}'"
        result = subprocess.run(cmd, shell=True, capture_output=True, text=True, timeout=10)
        return {"success": True, "matches": result.stdout[:5000]}
    except Exception as e:
        return {"success": False, "error": str(e)}

def tool_glob(pattern: str, path: str = ".") -> dict:
    """หาไฟล์ด้วย glob"""
    try:
        files = glob_module.glob(f"{path}/{pattern}", recursive=True)
        return {"success": True, "files": files[:100]}
    except Exception as e:
        return {"success": False, "error": str(e)}

def tool_webfetch(url: str) -> dict:
    """ดึงเนื้อหาเว็บ"""
    try:
        with httpx.Client(timeout=10) as client:
            resp = client.get(url)
            # ดึง text อย่างง่าย (ลบ HTML tags)
            text = re.sub(r'<[^>]+>', ' ', resp.text[:5000])
            text = re.sub(r'\s+', ' ', text).strip()
            return {"success": True, "content": text, "status": resp.status_code}
    except Exception as e:
        return {"success": False, "error": str(e)}

def tool_websearch(query: str) -> dict:
    """ค้นหาข้อมูลจากอินเทอร์เน็ต (DuckDuckGo)"""
    if not query:
        return {"success": False, "error": "Query is required"}
    
    try:
        headers = {
            "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"
        }
        # ใช้ DuckDuckGo HTML search (แบบไม่ต้องใช้ JavaScript)
        search_url = f"https://html.duckduckgo.com/html/?q={query}"
        
        with httpx.Client(timeout=15, headers=headers) as client:
            resp = client.get(search_url)
            if resp.status_code != 200:
                return {"success": False, "error": f"Search failed with status {resp.status_code}"}
            
            # สกัดข้อความอย่างง่าย (ลบ HTML tags และส่วนที่ไม่จำเป็น)
            text = re.sub(r'<style.*?>.*?</style>', ' ', resp.text, flags=re.DOTALL)
            text = re.sub(r'<script.*?>.*?</script>', ' ', text, flags=re.DOTALL)
            text = re.sub(r'<[^>]+>', ' ', text)
            text = re.sub(r'\s+', ' ', text).strip()
            
            return {"success": True, "content": text[:8000], "source": "DuckDuckGo"}
            
    except Exception as e:
        return {"success": False, "error": str(e)}

# ─── Tool Registry ──────────────────────────────────────────────────────────

TOOL_DEFINITIONS = [
    {
        "type": "function",
        "function": {
            "name": "read",
            "description": "Read file contents",
            "parameters": {
                "type": "object",
                "properties": {"path": {"type": "string", "description": "File path"}},
                "required": ["path"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "write",
            "description": "Create or overwrite a file",
            "parameters": {
                "type": "object",
                "properties": {
                    "path": {"type": "string", "description": "File path"},
                    "content": {"type": "string", "description": "File content"}
                },
                "required": ["path", "content"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "edit",
            "description": "Edit file contents by replacing a specific string with a new one",
            "parameters": {
                "type": "object",
                "properties": {
                    "path": {"type": "string", "description": "File path"},
                    "old_string": {"type": "string", "description": "The exact literal text to replace"},
                    "new_string": {"type": "string", "description": "The new text to replace it with"}
                },
                "required": ["path", "old_string", "new_string"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "list_directory",
            "description": "List files and directories in a path",
            "parameters": {
                "type": "object",
                "properties": {
                    "path": {"type": "string", "description": "The directory path to list", "default": "."},
                    "tree": {"type": "boolean", "description": "Whether to return a recursive tree view", "default": False}
                }
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "bash",
            "description": "Execute a bash command",
            "parameters": {
                "type": "object",
                "properties": {
                    "command": {"type": "string", "description": "Bash command"},
                    "timeout": {"type": "integer", "description": "Timeout in seconds", "default": 30}
                },
                "required": ["command"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "grep",
            "description": "Search file contents with regex",
            "parameters": {
                "type": "object",
                "properties": {
                    "pattern": {"type": "string", "description": "Regex pattern"},
                    "path": {"type": "string", "description": "File or directory to search", "default": "."},
                    "glob": {"type": "string", "description": "Glob filter (e.g. *.py)"}
                },
                "required": ["pattern"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "glob",
            "description": "Find files by glob pattern",
            "parameters": {
                "type": "object",
                "properties": {
                    "pattern": {"type": "string", "description": "Glob pattern (e.g. **/*.py)"},
                    "path": {"type": "string", "description": "Base directory", "default": "."}
                },
                "required": ["pattern"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "webfetch",
            "description": "Fetch a URL and extract text content",
            "parameters": {
                "type": "object",
                "properties": {"url": {"type": "string", "description": "URL to fetch"}},
                "required": ["url"]
            }
        }
    },
    {
        "type": "function",
        "function": {
            "name": "websearch",
            "description": "Search the web for information using a search engine",
            "parameters": {
                "type": "object",
                "properties": {"query": {"type": "string", "description": "The search query"}},
                "required": ["query"]
            }
        }
    },
]

TOOL_FUNCTIONS = {
    "read": lambda args: tool_read(args.get("path", "")),
    "write": lambda args: tool_write(args.get("path", ""), args.get("content", "")),
    "edit": lambda args: tool_edit(args.get("path", ""), args.get("old_string", ""), args.get("new_string", "")),
    "list_directory": lambda args: tool_list_directory(args.get("path", "."), args.get("tree", False)),
    "bash": lambda args: tool_bash(args.get("command", ""), args.get("timeout", 30)),
    "grep": lambda args: tool_grep(args.get("pattern", ""), args.get("path", "."), args.get("glob")),
    "glob": lambda args: tool_glob(args.get("pattern", ""), args.get("path", ".")),
    "webfetch": lambda args: tool_webfetch(args.get("url", "")),
    "websearch": lambda args: tool_websearch(args.get("query", "")),
}

# ─── KiloGateway Client ────────────────────────────────────────────────────

def call_kilogateway(messages: List[Dict], model: str = "kilo-auto/free", tools: List = None, mode: str = None) -> dict:
    """เรียก KiloGateway API"""
    api_key = os.environ.get("KILOCODE_API_KEY", "")
    
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json",
    }
    if mode:
        headers["x-kilocode-mode"] = mode
    
    body = {"model": model, "messages": messages}
    if tools:
        body["tools"] = tools
        body["tool_choice"] = "auto"
    
    with httpx.Client(timeout=120) as client:
        resp = client.post(
            "https://api.kilo.ai/api/gateway/chat/completions",
            headers=headers,
            json=body
        )
        
        if resp.status_code == 200:
            return resp.json()
        else:
            return {"error": resp.text, "status": resp.status_code}

# ─── Conversation Loop (Multi-turn with Tool Execution) ────────────────────

def run_conversation(prompt: str, model: str = "kilo-auto/free", mode: str = None, max_turns: int = 10) -> dict:
    """
    รัน conversation loop:
    1. ส่ง prompt ให้ AI
    2. ถ้า AI เรียก tool → execute → ส่งผลลัพธ์กลับ
    3. วนจนกว่า AI จะตอบข้อความสุดท้าย
    """
    messages = [{"role": "user", "content": prompt}]
    turn = 0
    
    print(f"🧠 Starting conversation: {prompt[:50]}...")
    
    while turn < max_turns:
        turn += 1
        print(f"🔄 Turn {turn}")
        
        # เรียก AI
        result = call_kilogateway(messages=messages, model=model, tools=TOOL_DEFINITIONS, mode=mode)
        
        if "error" in result:
            return {"success": False, "error": result.get("error"), "turns": turn}
        
        choices = result.get("choices", [])
        if not choices:
            return {"success": False, "error": "No choices in response", "raw": result}
        
        choice = choices[0]
        message = choice.get("message", {})
        finish_reason = choice.get("finish_reason", "")
        
        # เพิ่ม AI response เข้า messages
        messages.append({
            "role": "assistant",
            "content": message.get("content"),
            "tool_calls": message.get("tool_calls")
        })
        
        # 🔥 ถ้า AI เรียกใช้ tools
        tool_calls = message.get("tool_calls")
        if tool_calls and finish_reason == "tool_calls":
            print(f"🔧 AI called {len(tool_calls)} tool(s)")
            
            for tc in tool_calls:
                tc_id = tc.get("id", "")
                tc_function = tc.get("function", {})
                tc_name = tc_function.get("name", "")
                tc_args = json.loads(tc_function.get("arguments", "{}"))
                
                print(f"   📞 {tc_name}({json.dumps(tc_args)[:100]})")
                
                # Execute tool
                if tc_name in TOOL_FUNCTIONS:
                    try:
                        tool_result = TOOL_FUNCTIONS[tc_name](tc_args)
                        tool_output = json.dumps(tool_result, ensure_ascii=False)
                        print(f"   ✅ Result: {tool_output[:200]}")
                    except Exception as e:
                        tool_output = json.dumps({"error": str(e)})
                        print(f"   ❌ Error: {e}")
                else:
                    tool_output = json.dumps({"error": f"Unknown tool: {tc_name}"})
                    print(f"   ❌ Unknown tool: {tc_name}")
                
                # ส่ง tool result กลับ AI
                messages.append({
                    "role": "tool",
                    "tool_call_id": tc_id,
                    "content": tool_output[:10000]  # จำกัดขนาด
                })
            
            # วนรอบต่อไป - ส่ง tool results กลับ AI
            continue
        
        # 🔥 AI ตอบข้อความสุดท้าย (ไม่เรียก tool)
        ai_response = message.get("content", "")
        print(f"✅ Final response ({len(ai_response)} chars)")
        
        return {
            "success": True,
            "response": ai_response,
            "turns": turn,
            "messages": messages,
            "usage": result.get("usage", {})
        }
    
    return {"success": False, "error": f"Max turns ({max_turns}) reached"}

# ─── Webhook Endpoint ──────────────────────────────────────────────────────

@app.function(
    image=image,
    secrets=[
        modal.Secret.from_name("kilocode-secret"),
        modal.Secret.from_name("opencode-secret"),
    ],
    cpu=2,
    memory=2048,
    timeout=600,  # 10 นาที
)
@modal.fastapi_endpoint(method="POST")
def webhook(request: dict) -> dict:
    """
    KiloCode Gateway - รับ prompt แล้วรัน AI พร้อม tool calling
    """
    from starlette.responses import JSONResponse
    
    server_password = os.environ.get("KILO_SERVER_PASSWORD", "") or os.environ.get("OPENCODE_SERVER_PASSWORD", "")
    body = request if isinstance(request, dict) else {}
    
    # Enforce mandatory password validation
    if not server_password or body.get("password") != server_password:
        return JSONResponse(status_code=401, content={"error": "Unauthorized"})
    
    prompt = body.get("prompt") or body.get("message") or ""
    if not prompt:
        return JSONResponse(status_code=400, content={"error": "prompt required"})
    
    model = body.get("model") or "kilo-auto/free"
    mode = body.get("mode")
    max_turns = body.get("max_turns", 10)
    
    request_id = str(uuid.uuid4())
    print(f"📥 [{request_id}] {prompt[:80]}...")
    
    # 🔥 รัน conversation loop (AI + tool calling)
    result = run_conversation(
        prompt=prompt,
        model=model,
        mode=mode,
        max_turns=max_turns
    )
    
    if result.get("success"):
        return {
            "success": True,
            "requestId": request_id,
            "response": result.get("response"),
            "turns": result.get("turns"),
        }
    else:
        return JSONResponse(
            status_code=500,
            content={
                "success": False,
                "requestId": request_id,
                "error": result.get("error")
            }
        )
