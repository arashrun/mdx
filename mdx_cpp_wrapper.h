#ifndef MDX_CPP_WRAPPER_H
#define MDX_CPP_WRAPPER_H

#include "mdx_ffi.h"
#include <string>
#include <memory>

namespace mdx {

class Dictionary {
private:
    MdxHandle* handle_;

public:
    // 构造函数 - 打开 MDX 文件
    Dictionary(const std::string& file_path, const std::string& device_id = "") 
        : handle_(nullptr) {
        handle_ = mdx_open(file_path.c_str(), device_id.c_str());
    }

    // 析构函数 - 关闭 MDX 文件
    ~Dictionary() {
        if (handle_) {
            mdx_close(handle_);
        }
    }

    // 拷贝构造和赋值运算符被禁用，因为句柄不能安全地拷贝
    Dictionary(const Dictionary&) = delete;
    Dictionary& operator=(const Dictionary&) = delete;

    // 移动构造和赋值
    Dictionary(Dictionary&& other) noexcept : handle_(other.handle_) {
        other.handle_ = nullptr;
    }

    Dictionary& operator=(Dictionary&& other) noexcept {
        if (this != &other) {
            if (handle_) {
                mdx_close(handle_);
            }
            handle_ = other.handle_;
            other.handle_ = nullptr;
        }
        return *this;
    }

    // 查找关键词
    std::string lookup(const std::string& keyword) {
        if (!handle_) {
            return "";
        }

        char* result = mdx_lookup(handle_, keyword.c_str());
        if (result) {
            std::string definition(result);
            mdx_free_string(result); // 释放 Rust 分配的内存
            return definition;
        }
        return "";
    }

    // 检查关键词是否存在
    bool has_key(const std::string& keyword) {
        if (!handle_) {
            return false;
        }
        return mdx_has_key(handle_, keyword.c_str()) != 0;
    }

    // 获取词典标题
    std::string get_title() {
        if (!handle_) {
            return "";
        }

        char* result = mdx_get_title(handle_);
        if (result) {
            std::string title(result);
            mdx_free_string(result); // 释放 Rust 分配的内存
            return title;
        }
        return "";
    }

    // 获取词典描述
    std::string get_description() {
        if (!handle_) {
            return "";
        }

        char* result = mdx_get_description(handle_);
        if (result) {
            std::string desc(result);
            mdx_free_string(result); // 释放 Rust 分配的内存
            return desc;
        }
        return "";
    }

    // 检查句柄是否有效
    bool is_valid() const {
        return handle_ != nullptr;
    }
};

} // namespace mdx

#endif // MDX_CPP_WRAPPER_H