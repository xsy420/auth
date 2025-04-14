#include "auth/Color.hpp"
#include "auth/Cli.hpp"
#include <iostream>
#include <clocale>

int main(int argc, char* argv[]) {
    std::setlocale(LC_ALL, "");

    try {
        CAuthCLI cli;
        return cli.processCommand(argc, argv) ? 0 : 1;
    } catch (const std::exception& e) {
        std::cerr << CColor::RED << "Error: " << e.what() << CColor::RESET << std::endl;
        return 1;
    }
}