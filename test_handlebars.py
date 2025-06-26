#!/usr/bin/env python3
import json

# 模拟模板文本和数据
template_text = "员工姓名: {{employee.name}}部门: {{employee.department}}入职日期: {{employee.hire_date}}项目经历:{{#each projects}}- {{name}}: {{description}} ({{status}}){{/each}}{{#if employee.has_bonus}}奖金: ¥{{employee.bonus_amount}}{{/if}}"

data = {
    "employee": {
        "name": "陈小华",
        "department": "产品部",
        "position": "产品经理",
        "hire_date": "2024-02-20",
        "has_bonus": True,
        "bonus_amount": 12000,
        "email": "chenxiaohua@company.com"
    },
    "company": {
        "name": "创新科技有限公司",
        "address": "上海市浦东新区张江高科技园区",
        "industry": "人工智能"
    },
    "projects": [
        {
            "name": "AI助手平台",
            "description": "智能对话系统产品设计",
            "status": "已上线",
            "duration": "3个月",
            "team_size": 8
        },
        {
            "name": "数据分析工具",
            "description": "用户行为分析平台",
            "status": "开发中",
            "duration": "2个月",
            "team_size": 5
        },
        {
            "name": "移动应用重构",
            "description": "用户体验优化项目",
            "status": "规划中",
            "duration": "4个月",
            "team_size": 12
        }
    ]
}

print("原始模板:")
print(repr(template_text))
print()
print("原始模板 (可读):")
print(template_text)
print()

# 使用 Python 的 pystache 模拟 Handlebars 渲染
try:
    import pystache
    renderer = pystache.Renderer()
    
    # 修改模板为 Mustache 格式 (相似但不完全相同)
    mustache_template = template_text.replace("{{#each projects}}", "{{#projects}}").replace("{{/each}}", "{{/projects}}")
    mustache_template = mustache_template.replace("{{#if employee.has_bonus}}", "{{#employee.has_bonus}}").replace("{{/if}}", "{{/employee.has_bonus}}")
    
    print("Mustache 模板:")
    print(mustache_template)
    print()
    
    rendered = renderer.render(mustache_template, data)
    print("预期渲染结果:")
    print(repr(rendered))
    print()
    print("预期渲染结果 (可读):")
    print(rendered)
except ImportError:
    print("未安装 pystache，跳过渲染测试")
    print("手动预期结果应该是:")
    print("员工姓名: 陈小华部门: 产品部入职日期: 2024-02-20项目经历:- AI助手平台: 智能对话系统产品设计 (已上线)- 数据分析工具: 用户行为分析平台 (开发中)- 移动应用重构: 用户体验优化项目 (规划中)奖金: ¥12000")
