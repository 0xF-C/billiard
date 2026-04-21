"""
RS485 继电器控制器
通过 pymodbus 与 RS485 继电器模块通信
台球桌编号 ↔ 继电器地址映射: table_id -> relay_addr = table_id + 1
"""
import sys
import os

# 将 hardware 目录加入路径
sys.path.insert(0, os.path.dirname(__file__))

try:
    from pymodbus.client import ModbusTcpClient
    MODBUS_AVAILABLE = True
except ImportError:
    MODBUS_AVAILABLE = False
    print("[硬件] pymodbus 未安装，继电器功能不可用。安装: pip install pymodbus")


# 继电器模块配置
DEFAULT_HOST = "192.168.1.100"
DEFAULT_PORT = 502


def _get_client():
    """创建 Modbus TCP 客户端"""
    if not MODBUS_AVAILABLE:
        raise RuntimeError("pymodbus 未安装")
    # 从 Flask config 读取配置（如果可用）
    try:
        from config import Config
        host = Config.MODBUS_HOST
        port = Config.MODBUS_PORT
    except ImportError:
        host = DEFAULT_HOST
        port = DEFAULT_PORT

    client = ModbusTcpClient(host, port=port, timeout=3)
    connected = client.connect()
    if not connected:
        raise ConnectionError(f"无法连接继电器模块 {host}:{port}")
    return client


def _table_to_relay(table_id):
    """台球桌 ID → 继电器地址"""
    return table_id + 1


def turn_on(table_id):
    """开灯：打开对应继电器"""
    client = _get_client()
    addr = _table_to_relay(table_id)
    # 写单个线圈为 ON (0xFF00)
    client.write_coil(addr, True, slave=1)
    client.close()
    print(f"[继电器] 台球桌 #{table_id} -> 继电器地址 {addr} -> 开灯 ✅")


def turn_off(table_id):
    """关灯：关闭对应继电器"""
    client = _get_client()
    addr = _table_to_relay(table_id)
    client.write_coil(addr, False, slave=1)
    client.close()
    print(f"[继电器] 台球桌 #{table_id} -> 继电器地址 {addr} -> 关灯 ✅")


def get_all_status():
    """读取所有继电器状态（12桌）"""
    client = _get_client()
    # 读取 12 个线圈状态
    result = client.read_coils(1, 12, slave=1)
    client.close()

    status = {}
    if not result.isError():
        for i in range(12):
            status[i + 1] = result.bits[i]  # table_id 1-12
    return status


def test_connection():
    """测试连接并读取一次数据"""
    client = _get_client()
    result = client.read_coils(1, 1, slave=1)
    client.close()
    if result.isError():
        return {"connected": False, "error": str(result)}
    return {"connected": True, "coil_1": result.bits[0]}
