#include <iostream>

#include "tealang.hpp"

using namespace tea;

int main() {
    auto tenv = TeaLang::new_env();

    for(;;) {
        std::string exp;

        std::cout << ">" << std::flush;
        if ( !std::getline(std::cin, exp) ) {
            break;
        }

        std::cout << TeaLang::run(exp, tenv) << std::endl;
    }

}
