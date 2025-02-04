/* 
 * File:   Parser.h
 * Author: tg
 *
 * Created on December 1, 2015, 11:45 AM
 */

#ifndef PARSER_H
#define	PARSER_H

#include <string>
#include <vector>

/**
 * Simplified command line parser
 * @param orig
 */
class Parser {
private:
    // name of input sum file
    std::string sumfile_;
    // alternatively, provide parameters in separate file
    std::string paramsfile_;
    // file with offsets for Stadivari
    std::string offsetsfile_;
    // template for XDS.INP
    std::string xdsinptemplate_;
    // explicitly provide run number for stoe
    unsigned short stoerun_;
    // output directory (default: current dir/run??)
    std::string outdir_;
    // verbosity level
    unsigned char verbosity_;
    
    template <typename T> bool getoption(const std::string& option,
            const std::string& opt, T& optval, int& idx, int argc,
            const char* const argv[]);

public:
    Parser(int argc, const char* const argv[]);
    ~Parser(){};
    
    // return the basename of the program
    std::string myname();
    std::string sumfile() const {return sumfile_;}
    std::string paramsfile() const { return paramsfile_; }
    std::string offsetsfile() const { return offsetsfile_; }
    std::string xdsinptemplatefile() const { return xdsinptemplate_; }
    unsigned short stoerun() const { return stoerun_; }
    unsigned char verbosity() const { return verbosity_; }
};

#endif	/* PARSER_H */

