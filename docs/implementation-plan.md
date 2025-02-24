# Sync-Pointer 实现方案

## 1. 系统总览

### 1.1 项目目标

实现局域网内多台计算机共享键鼠的功能，提供流畅的用户体验和可靠的连接机制。

### 1.2 系统架构

```
+----------------+        +----------------+
|    控制端      |        |    被控端      |
|  (Server)      |        |  (Client)      |
+----------------+        +----------------+
| - 输入捕获     |        | - 输入事件处理  |
| - 设备发现     | -----> | - 状态同步     |
| - 事件转发     |        | - 边界检测     |
| - 状态管理     |        | - 异常恢复     |
+----------------+        +----------------+
```

## 2. 核心功能设计

### 2.1 网络通信设计

#### 传输协议

1. 混合传输模式

   - TCP通道: 可靠性要求高的数据
     - 连接控制命令
     - 配置同步
     - 键盘事件
     - 剪贴板数据
   - UDP通道: 实时性要求高的数据
     - 鼠标移动事件
     - 状态同步
     - 心跳包

2. 消息结构

```
+----------------+
|    消息头      |
+----------------+
| 类型(1B)      |
| 序列号(4B)    |
| 时间戳(8B)    |
| 负载长度(4B)  |
+----------------+
|    消息体      |
+----------------+
```

#### 安全机制

1. 加密方案
   - 默认: AES-128-GCM + ECDH
   - 可选: 无加密/高级加密(AES-256)
2. 密钥管理
   - 动态密钥交换(ECDH)
   - 会话密钥定期轮换
   - 可选预共享密钥(PSK)

### 2.2 输入事件处理

#### 键盘事件

```rust
pub struct KeyboardManager {
    // 按键状态记录
    pressed_keys: HashSet<KeyCode>,

    // 组合键处理
    combo_detector: ComboDetector,

    // 事件发送通道
    event_sender: mpsc::Sender<KeyboardEvent>,
}

impl KeyboardManager {
    // 事件捕获与处理
    pub fn handle_event(&mut self, event: KeyboardEvent) -> Result<()> {
        // 更新按键状态
        match event.event_type {
            KeyEventType::Press => self.pressed_keys.insert(event.key_code),
            KeyEventType::Release => self.pressed_keys.remove(&event.key_code),
        }

        // 检测组合键
        if let Some(combo) = self.combo_detector.check(&self.pressed_keys) {
            self.handle_combo(combo)?;
        }

        // 发送事件
        self.event_sender.send(event)?;

        Ok(())
    }
}
```

#### 鼠标事件

```rust
pub struct MouseManager {
    // 当前位置
    current_position: Point,

    // 按键状态
    button_states: MouseButtonState,

    // 边界检测器
    boundary_detector: BoundaryDetector,

    // 事件发送通道
    event_sender: mpsc::Sender<MouseEvent>,
}

impl MouseManager {
    // 处理鼠标移动
    pub fn handle_move(&mut self, new_position: Point) -> Result<()> {
        // 检查边界过渡
        if let Some(transition) = self.boundary_detector.check_transition(
            self.current_position,
            new_position,
        ) {
            self.handle_transition(transition)?;
            return Ok(());
        }

        // 更新位置
        self.current_position = new_position;

        // 发送移动事件
        self.event_sender.send(MouseEvent::Move(new_position))?;

        Ok(())
    }
}
```

### 2.3 异常处理机制

#### 设备断开处理

```rust
pub struct DisconnectionHandler {
    // 位置管理器
    position_manager: MousePositionManager,

    // 状态管理器
    state_manager: InputStateManager,

    // 重连管理器
    reconnection_manager: ReconnectionManager,
}

impl DisconnectionHandler {
    // 处理设备断开
    pub async fn handle_device_disconnect(&mut self, device_id: &str) -> Result<()> {
        // 1. 恢复鼠标位置
        self.position_manager.handle_device_disconnect(device_id)?;

        // 2. 清理输入状态
        self.state_manager.handle_disconnect(device_id)?;

        // 3. 尝试重连
        self.reconnection_manager.handle_disconnect(device_id).await?;

        Ok(())
    }
}
```

#### 网络异常恢复

```rust
pub struct NetworkRecovery {
    // 连接监控
    connection_monitor: ConnectionMonitor,

    // 状态同步器
    state_syncer: StateSynchronizer,
}

impl NetworkRecovery {
    // 执行恢复流程
    pub async fn perform_recovery(&mut self, device_id: &str) -> Result<()> {
        // 1. 重建连接
        self.connection_monitor.reconnect(device_id).await?;

        // 2. 同步状态
        self.state_syncer.sync_states(device_id).await?;

        Ok(())
    }
}
```

### 2.4 用户界面设计

#### 设备管理界面

```typescript
interface DeviceManager {
  // 设备列表
  devices: Device[];

  // 设备操作
  connectDevice: (deviceId: string) => Promise<void>;
  disconnectDevice: (deviceId: string) => Promise<void>;
  configureDevice: (deviceId: string, config: DeviceConfig) => Promise<void>;

  // 状态展示
  showDeviceStatus: (deviceId: string) => void;
  showConnectionStatus: () => void;
}
```

#### 配置界面

```typescript
interface Settings {
  // 网络设置
  network: {
    transportMode: 'tcp-only' | 'udp-only' | 'hybrid';
    encryption: {
      mode: 'none' | 'basic' | 'advanced';
      keyManagement: 'psk' | 'dynamic';
    };
  };

  // 输入设置
  input: {
    mouseTransition: {
      mode: 'instant' | 'smooth';
      animationDuration: number;
    };
    keyboardConfig: {
      preventKeys: string[];
      combos: KeyCombo[];
    };
  };

  // 异常处理
  errorHandling: {
    reconnect: {
      enabled: boolean;
      maxAttempts: number;
      interval: number;
    };
    notifications: {
      showDisconnect: boolean;
      showReconnect: boolean;
    };
  };
}
```

## 3. 实现计划

### 3.1 后端实现(Rust)

1. 核心模块

   ```
   src-tauri/src/core/
   ├── input/
   │   ├── keyboard.rs    // 键盘事件处理
   │   ├── mouse.rs       // 鼠标事件处理
   │   └── mod.rs
   ├── network/
   │   ├── protocol.rs    // 通信协议
   │   ├── tcp.rs         // TCP管理器
   │   ├── udp.rs         // UDP管理器
   │   └── mod.rs
   └── state/
       ├── device.rs      // 设备状态
       ├── input.rs       // 输入状态
       └── mod.rs
   ```

2. 服务模块
   ```
   src-tauri/src/service/
   ├── server/
   │   ├── mdns.rs        // 设备发现
   │   ├── input.rs       // 输入服务
   │   └── mod.rs
   ├── client/
   │   ├── mdns.rs        // 设备发现
   │   ├── input.rs       // 输入处理
   │   └── mod.rs
   └── common/
       ├── crypto.rs      // 加密服务
       ├── clipboard.rs   // 剪贴板同步
       └── mod.rs
   ```

### 3.2 前端实现(React + TypeScript)

1. 页面组件
   ```
   src/pages/
   ├── devices/
   │   ├── DeviceList.tsx
   │   ├── DeviceCard.tsx
   │   └── DeviceConfig.tsx
   ├── layout/
   │   ├── ScreenLayout.tsx
   │   └── BoundaryConfig.tsx
   └── settings/
       ├── NetworkSettings.tsx
       ├── InputSettings.tsx
       └── SystemSettings.tsx
   ```

## 4. 测试计划

### 4.1 单元测试

1. 输入事件处理

   - 键盘事件捕获
   - 鼠标事件处理
   - 组合键检测

2. 网络通信
   - 协议实现
   - 加密功能
   - 断线重连

### 4.2 集成测试

1. 端到端流程

   - 设备发现连接
   - 输入控制传输
   - 状态同步

2. 异常场景
   - 网络中断恢复
   - 设备突然断开
   - 并发操作处理

### 4.3 性能测试

1. 延迟测试

   - 输入响应时间
   - 网络传输延迟
   - 状态同步延迟

2. 压力测试
   - 高频事件处理
   - 多设备并发
   - 长时间运行

## 5. 发布计划

### 5.1 版本规划

1. v0.1.0 - 基础功能

   - 设备发现与连接
   - 基本键鼠控制
   - 简单界面

2. v0.2.0 - 核心功能

   - 完整键鼠控制
   - 剪贴板同步
   - 配置界面

3. v0.3.0 - 功能完善

   - 加密支持
   - 异常处理
   - 性能优化

4. v1.0.0 - 正式版本
   - 功能完整
   - 稳定可靠
   - 文档完善

### 5.2 质量目标

1. 性能指标

   - 输入延迟 < 20ms
   - CPU使用率 < 5%
   - 内存占用 < 50MB

2. 可靠性指标
   - 崩溃率 < 0.1%
   - 重连成功率 > 99%
   - 数据丢失率 < 0.01%

## 6. 注意事项

### 6.1 安全考虑

1. 本地通信加密
2. 输入验证
3. 权限控制
4. 异常处理

### 6.2 兼容性

1. 操作系统兼容

   - Windows 10/11
   - macOS 10.15+
   - 主流Linux发行版

2. 硬件兼容
   - 高DPI支持
   - 多显示器支持
   - 不同输入设备

### 6.3 性能优化

1. 事件批处理
2. 内存管理
3. CPU使用优化
4. 网络带宽优化
