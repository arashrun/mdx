# 构建 MDX FFI 库

本项目提供了 Rust 的 FFI 接口，可以被 C/C++ 程序调用。以下是构建步骤：

## 1. 安装 Rust 工具链

在构建之前，您需要安装 Rust 工具链：

```bash
# 安装 Rust (Windows)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# 或者在 Windows 上使用
winget install Rustlang.Rustup

# 安装完成后，重新加载环境变量或重启终端
source ~/.bashrc  # Linux/macOS
# 或重启终端
```

## 2. 构建动态链接库

```bash
# 构建发布版本的动态链接库
cargo build --release

# 构建完成后，动态库文件将位于：
# - Linux: target/release/libmdx.so
# - macOS: target/release/libmdx.dylib
# - Windows: target/release/mdx.dll
```

## 3. 使用 C++ 封装

头文件和 C++ 封装类已提供：

- `mdx_ffi.h`: C 语言接口定义
- `mdx_cpp_wrapper.h`: C++ 封装类
- `example.cpp`: 使用示例

### 编译 C++ 示例程序

```bash
# 假设 Rust 库已构建，编译 C++ 示例
# Linux/macOS:
g++ -std=c++11 -I. example.cpp -L./target/release -lmdx -o example

# Windows (使用 MSVC 工具链):
cl /I. example.cpp /link /LIBPATH:.\target\release mdx.lib /OUT:example.exe
```

## 4. FFI 接口说明

### C 接口

- `MdxHandle* mdx_open(const char* file_path, const char* device_id)`: 打开 MDX 文件
- `void mdx_close(MdxHandle* handle)`: 关闭 MDX 文件
- `char* mdx_lookup(MdxHandle* handle, const char* keyword)`: 查找关键词
- `int mdx_has_key(MdxHandle* handle, const char* keyword)`: 检查关键词是否存在
- `void mdx_free_string(char* s)`: 释放字符串内存
- `char* mdx_get_title(MdxHandle* handle)`: 获取词典标题
- `char* mdx_get_description(MdxHandle* handle)`: 获取词典描述

### C++ 封装类

使用 `mdx::Dictionary` 简化操作：

```cpp
#include "mdx_cpp_wrapper.h"
#include <iostream>

int main() {
    // 创建字典实例
    mdx::Dictionary dict("path/to/your/dictionary.mdx", "device_id");

    if (!dict.is_valid()) {
        std::cerr << "Failed to open dictionary file" << std::endl;
        return 1;
    }

    // 查找单词
    std::string definition = dict.lookup("hello");
    if (!definition.empty()) {
        std::cout << "Definition: " << definition << std::endl;
    }

    return 0;
}
```

## 5. 注意事项

1. 字符串内存管理：由 `mdx_lookup`、`mdx_get_title` 和 `mdx_get_description` 返回的字符串
   必须使用 `mdx_free_string` 释放，否则会导致内存泄漏。

2. 线程安全：当前 FFI 接口不是线程安全的，如需多线程访问，请添加适当的同步机制。

3. 错误处理：所有接口在失败时都有适当的返回值，使用前请检查句柄是否有效。

4. 编码：传入的路径和关键词字符串应使用 UTF-8 编码。