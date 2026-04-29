#!/usr/bin/env python3
"""OlaForge CLI - Python SDK 命令行工具"""

import sys
import argparse
from olaforge import OlaForge

def main():
    parser = argparse.ArgumentParser(description="OlaForge - AI Agent 安全沙箱")
    parser.add_argument("command", choices=["exec", "run", "skills", "logs", "audit", "health"])
    parser.add_argument("code", nargs="?", help="要执行的代码")
    parser.add_argument("-l", "--language", default="python", help="语言")
    parser.add_argument("-s", "--security", default="L2", help="安全级别")
    parser.add_argument("-t", "--timeout", type=int, default=60, help="超时")
    
    args = parser.parse_args()
    client = OlaForge()
    
    if args.command == "exec":
        if not args.code:
            print("错误: 需要提供代码", file=sys.stderr)
            sys.exit(1)
        result = client.execute(args.code, args.language, args.security, args.timeout)
        print(result.output)
        sys.exit(0 if result.success else 1)
    
    elif args.command == "health":
        print(f"健康: {client.health_check()}")
    
    elif args.command == "skills":
        skills = client.list_skills()
        for s in skills:
            print(f"- {s.name}: {s.description}")
    
    elif args.command == "logs":
        logs = client.get_logs(10)
        for log in logs:
            status = "✓" if log.get("success") else "✗"
            print(f"[{status}] {log.get('language')} - {log.get('execution_time_ms')}ms")
    
    elif args.command == "audit":
        print("请指定路径: olaforge audit <path>")
    
    elif args.command == "run":
        print("请指定技能目录: olaforge run <skill_dir>")

if __name__ == "__main__":
    main()