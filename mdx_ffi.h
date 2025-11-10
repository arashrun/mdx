#ifndef MDX_FFI_H
#define MDX_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

// MDX 句柄类型
typedef struct MdxHandle MdxHandle;

// 打开 MDX 文件并返回句柄
// 参数:
//   file_path: MDX 文件路径 (UTF-8 编码)
//   device_id: 设备 ID (用于解密)
// 返回值: 成功时返回句柄，失败时返回 NULL
MdxHandle* mdx_open(const char* file_path, const char* device_id);

// 关闭 MDX 文件并释放资源
void mdx_close(MdxHandle* handle);

// 查找关键词并返回 HTML 定义
// 注意: 调用者负责使用 mdx_free_string 释放返回的字符串
// 参数:
//   handle: MDX 句柄
//   keyword: 要查找的关键词
// 返回值: 成功时返回定义字符串，失败时返回 NULL
char* mdx_lookup(MdxHandle* handle, const char* keyword);

// 检查关键词是否存在
// 参数:
//   handle: MDX 句柄
//   keyword: 要检查的关键词
// 返回值: 存在返回 1，不存在返回 0
int mdx_has_key(MdxHandle* handle, const char* keyword);

// 释放由 mdx_lookup 返回的字符串
void mdx_free_string(char* s);

// 获取词典标题
// 注意: 调用者负责使用 mdx_free_string 释放返回的字符串
char* mdx_get_title(MdxHandle* handle);

// 获取词典描述
// 注意: 调用者负责使用 mdx_free_string 释放返回的字符串
char* mdx_get_description(MdxHandle* handle);

#ifdef __cplusplus
}
#endif

#endif // MDX_FFI_H