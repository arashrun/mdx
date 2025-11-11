#include "mdx.h"
#include <iostream>
using std::cout;
using std::endl;
using std::string;

int main(int argc, char** argv)
{
    // 创建字典实例，需要提供 MDX 文件路径
    // 注意：你需要提供一个实际存在的 MDX 文件路径
    if (argc < 2) {
        cout << "usage: " << argv[0] << " <mdx>" << endl;
        return -1;
    }
    std::string mdx(argv[1]);
    mdx::Dictionary dict(mdx, "device_id");

    if (!dict.is_valid()) {
        std::cerr << "Failed to open dictionary file" << std::endl;
        return 1;
    }

    std::cout << "Dictionary Title: " << dict.get_title() << std::endl;
    std::cout << "Dictionary Description: " << dict.get_description() << std::endl;

    // 查找单词
    std::string keyword = "hello";
    std::string definition = dict.lookup(keyword);

    if (!definition.empty()) {
        std::cout << "Definition of '" << keyword << "': " << std::endl;
        std::cout << definition << std::endl;
    } else {
        std::cout << "Keyword '" << keyword << "' not found." << std::endl;
    }

    // 检查关键词是否存在
    if (dict.has_key("world")) {
        std::cout << "Keyword 'world' exists in the dictionary." << std::endl;
    } else {
        std::cout << "Keyword 'world' does not exist in the dictionary." << std::endl;
    }

    string word;
    string prompt("(mdx)>>>");
    cout << prompt;
    while (std::cin >> word) {
        if (word == "quit")
            break;
        cout << dict.lookup(word) << endl;
        // cout << word << endl;
        cout << prompt;
    }
    cout << "!!![finish]!!!" << endl;

    return 0;
}
