# 🎱 台球厅智能管理系统

> 开台即亮灯，结账即灭灯 — 一站式台球厅经营管理工具

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | Vue 3 + Vite + Pinia + Element Plus + Vue Router |
| 后端 | Python 3.9+ / Flask + Flask-SQLAlchemy |
| 数据库 | SQLite（零配置，单文件） |
| 硬件 | pymodbus（RS485 继电器模块通信） |

## 项目结构

```
billiard-manager/
├── backend/                # Flask 后端
│   ├── app.py              # 入口（应用工厂模式）
│   ├── config.py           # 配置文件
│   ├── models.py           # 数据库模型（Table/Order/Member）
│   ├── routes/             # API 路由蓝图
│   │   ├── table_routes.py    # 台球桌 CRUD
│   │   ├── order_routes.py    # 开台/结账/订单
│   │   ├── member_routes.py   # 会员管理/充值
│   │   ├── stats_routes.py    # 数据统计/看板
│   │   └── hardware_routes.py # 继电器控制
│   └── requirements.txt    # Python 依赖
├── frontend/               # Vue 3 前端
│   ├── src/
│   │   ├── api/               # Axios 请求封装
│   │   ├── views/             # 页面组件
│   │   ├── components/        # 公共组件（Layout 侧边栏）
│   │   ├── router/            # 路由配置
│   │   └── main.js            # 入口
│   └── package.json
├── hardware/               # 硬件交互
│   └── relay_controller.py # RS485 继电器控制器
└── README.md
```

## 快速启动

### 1. 启动后端

```bash
cd backend
pip3 install -r requirements.txt
python3 app.py
# 后端运行在 http://localhost:5000
```

### 2. 启动前端

```bash
cd frontend
npm install
npm run dev
# 前端运行在 http://localhost:5173
```

## 核心功能

- **一键开台**：选桌 + 选会员（可选散客），开台自动开灯
- **智能计时**：精确到分钟，自动按桌型计费
- **快捷结账**：自动计算时长/金额，支持会员折扣和储值扣款
- **会员体系**：手机号注册、充值、折扣系数、消费记录
- **数据看板**：今日营收、订单数、使用率、7天趋势图
- **硬件联动**：RS485 继电器模块，开台亮灯 / 结账灭灯

## API 概览

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/api/tables` | 获取所有台球桌 |
| POST | `/api/orders/open` | 开台 |
| POST | `/api/orders/<id>/close` | 结账 |
| GET | `/api/orders/active` | 进行中订单 |
| GET | `/api/members` | 会员列表 |
| POST | `/api/members` | 创建会员 |
| POST | `/api/members/<id>/recharge` | 充值 |
| GET | `/api/stats/dashboard` | 看板数据 |
| GET | `/api/hardware/status` | 继电器状态 |

## 默认数据

首次启动自动创建 12 张台球桌：
- **1~8号桌**（普通）：¥0.5/分钟
- **VIP-A ~ VIP-D**（VIP）：¥1.0/分钟

## 注意事项

- 硬件功能需要 RS485 继电器模块在线，否则不影响软件使用（自动降级）
- 修改继电器 IP/端口请在 `backend/config.py` 中调整 `MODBUS_HOST` 和 `MODBUS_PORT`
