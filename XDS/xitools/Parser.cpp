/* 
 * File:   Parser.cpp
 * Author: tg
 * 
 * Created on December 1, 2015, 11:45 AM
 */

#include "Parser.h"
#include "myExceptions.h"
#include <iostream>

Parser::Parser(int argc, const char* const argv[]) :
sumfile_(""),
paramsfile_(""),
offsetsfile_(""),
xdsinptemplate_(""),
stoerun_(0),
        outdir_("./"),
verbosity_(0) {
    
    for (int i = 1; i < argc; ++i) {
        std::string option = argv[i];
        if (option.substr(0, 2) == "-h" || option.substr(0, 2) == "-?") {
            throw myExcepts::Usage("Help message");
            return;
        }
        if (getoption(option, "-p", paramsfile_, i, argc, argv)) continue;
        if (getoption(option, "-O", offsetsfile_, i, argc, argv)) continue;
        if (getoption(option, "-r", stoerun_, i, argc, argv)) continue;
        if (getoption(option, "-o", outdir_, i, argc, argv)) continue;
        if (getoption(option, "-t", xdsinptemplate_, i, argc, argv)) continue;
        if (getoption(option, "-v", verbosity_, i, argc, argv)) continue;
        if (getoption(option, "-x", sumfile_, i, argc, argv)) continue; 

        // when reaching this points, unknown option character
        if (option.at(0) == '-') {
            std::cout << "Unknown option string " << option << std::endl;
            throw myExcepts::Usage(option.c_str());
        }
    }
    if (sumfile_.empty()) {
        std::cout << "*** Error: please provide input file name.\n";
        throw myExcepts::Usage("Missing filename");
    }
}

/**
 * Interpret option string
 * @param option
 * @param opt
 * @param optval
 * @param idx
 * @param argc
 * @param argv
 * @return 
 */
template <typename T> bool Parser::getoption(const std::string& option,
        const std::string& opt, T& optval, int& idx, int argc,
        const char* const argv[]) {

    if (option.substr(0, 2) != opt) return false;

    std::string convertee;
    if (option.length() > 2) {
        convertee = option.substr(2, option.length());
    } else {
        if (idx + 1 >= argc) {
            std::cout << "*** Error: option " << argv[idx] << " requires "
                    << "an argument.\n";
            throw myExcepts::Usage("Option requires argument");
        }
        convertee = argv[idx + 1];
        ++idx;
    }
    std::istringstream inp(convertee);
    inp >> optval;
    if (inp.fail()) {
        std::cout << "*** Error: Cannot convert " << convertee
                << std::endl;
        throw myExcepts::Usage("Conversion of command line parameter");
    }
    return true;
}