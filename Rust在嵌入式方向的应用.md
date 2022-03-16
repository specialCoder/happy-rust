# 机器人平台的 Rust 资源
在将 Rust 用于机器人平台的开发方面，有一个小型的，但正在不断增长的公司和开发人员社区（包括 Tangram Vision）。他们正在将一些机器人平台最常用的库和工具与 Rust 相融合，并且也在开发新的工具，以简化创建 Rust-编程（Rust-programmed）机器人的开发路径。

## 框架
- [OpenRR](https://github.com/openrr/openrr)：开源的 Rust 机器人平台
  
## ROS
- [rosrust](https://github.com/adnanademovic/rosrust)：完全由 Rust 实现的 ROS 客户端库
- [ros2-rust](https://github.com/ros2-rust/ros2_rust)：ROS2 的 Rust 绑定、代码生成器，以及示例代码
- [rustros_tf](https://github.com/arjo129/rustros_tf)：ROS tf 库的 Rust 端口，用于追踪三维变换
- [Optimization Engine](https://github.com/alphaville/optimization-engine)：用于机器人和自主系统的嵌入式优化

## 计算机视觉
- [realsense-rust](https://docs.rs/crate/realsense-rust/0.5.1)：用于 Intel RealSense 深度摄像机的高级绑定（Tangram Vision 维护）
- [opencv-ros-camera](https://lib.rs/crates/opencv-ros-camera): 一种兼容 OpenCV 的相机几何模型
- [adskalman](https://lib.rs/crates/adskalman): 卡尔曼（Kalman）电子滤波器
- [cam-geom](https://lib.rs/crates/cam-geom): 相机几何模型
- [bayes_estimate](https://lib.rs/crates/bayes_estimate): 一种贝叶斯（Bayesian）评估库

## 碰撞侦测
- [openrr-planner](https://lib.rs/crates/openrr-planner): 规避性的路径规划

## 控制器
- [stepper](https://lib.rs/crates/stepper): Rust 的通用步进电机驱动器和控制器接口

## 模拟器
- [nphysics](https://github.com/dimforge/nphysics): 可用于机器人仿真的 2D 和 3D 物理引擎

## 数学计算
- [Nalgebra](https://www.nalgebra.org/): Rust 的线性代数计算库
- [petgraph](https://github.com/petgraph/petgraph): 图数据结构库，兼容 Rust