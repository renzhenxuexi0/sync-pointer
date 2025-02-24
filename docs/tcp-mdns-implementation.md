# TCP和MDNS实现计划

## 1. TCP服务实现

### 1.1 服务端实现
- [ ] 创建 TCP 服务器模块 `src-tauri/src/service/server/tcp.rs`
  - 实现 TCP 监听器
  - 处理客户端连接
  - 实现并发连接管理
  - 添加心跳检测机制
  - 实现错误处理和重连逻辑

### 1.2 客户端实现
- [ ] 创建 TCP 客户端模块 `src-tauri/src/service/client/tcp.rs`
  - 实现 TCP 连接管理
  - 添加重连机制
  - 实现心跳检测
  - 处理连接状态变化

### 1.3 通信协议
- [ ] 定义消息类型 `src-tauri/src/protocol/mod.rs`
  ```rust
  enum MessageType {
      Connect,
      Disconnect,
      Heartbeat,
      MouseMove,
      KeyPress,
      ClipboardSync,
  }
  ```

## 2. MDNS服务完善

### 2.1 服务注册
- [ ] 完善 MDNS 服务器 `src-tauri/src/service/server/mdns.rs`
  - 注册服务类型 "_sync-pointer._tcp"
  - 添加服务元数据（设备名称、IP、端口等）
  - 实现服务状态更新

### 2.2 服务发现
- [ ] 完善 MDNS 客户端 `src-tauri/src/service/client/mdns.rs`
  - 实现服务发现逻辑
  - 处理服务上线/下线事件
  - 缓存发现的服务信息

## 3. 前端集成

### 3.1 API接口
- [ ] 创建前端 TCP API `src/api/tcp.ts`
  - 连接管理接口
  - 状态查询接口
  - 错误处理接口

### 3.2 状态管理
- [ ] 更新设备状态管理 `src/store/devices/index.ts`
  - 添加连接状态
  - 添加设备在线状态
  - 实现状态监听和更新

### 3.3 UI组件
- [ ] 更新设备网格组件 `src/pages/screen-layout/components/DeviceGrid.tsx`
  - 显示连接状态
  - 添加连接操作按钮
  - 显示错误信息

## 4. 测试计划

### 4.1 单元测试
- [ ] TCP服务端测试
- [ ] TCP客户端测试
- [ ] MDNS服务测试
- [ ] 协议测试

### 4.2 集成测试
- [ ] 多设备连接测试
- [ ] 网络异常恢复测试
- [ ] 性能压力测试

## 5. 性能优化

### 5.1 传输优化
- [ ] 实现消息压缩
- [ ] 使用二进制协议
- [ ] 优化心跳间隔

### 5.2 并发优化
- [ ] 使用异步IO
- [ ] 实现连接池
- [ ] 优化资源释放

## 6. 安全措施

### 6.1 通信安全
- [ ] 实现TLS加密
- [ ] 添加身份验证
- [ ] 实现访问控制

### 6.2 错误处理
- [ ] 完善错误日志
- [ ] 实现优雅降级
- [ ] 添加自动恢复机制

## 实施顺序

1. 首先实现基础TCP服务端和客户端
2. 完善MDNS服务发现功能
3. 整合前端状态管理和UI
4. 添加测试用例
5. 进行性能优化
6. 实现安全措施

## 注意事项

1. 确保所有网络操作都是异步的
2. 实现proper错误处理和日志记录
3. 保持代码模块化和可测试性
4. 考虑跨平台兼容性
5. 遵循Rust和TypeScript的最佳实践