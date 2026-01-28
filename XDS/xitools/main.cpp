/* 
 * File:   main.cpp
 * Author: tg
 *
 * Created on November 6, 2013, 12:32 PM
 */

#include <stdexcept>
#include <iostream>

#include "Usage.h"
#include "Parser.h"
#include "myExceptions.h"

#include "io.h"
#include "StoeSum.h"

/*
 * 
 */
int main(int argc, char** argv) {
    Parser* parser = 0;
    hello(std::cout);
    try {
        parser = new Parser(argc, argv);
    } catch (myExcepts::Usage& e) {
        std::cout << e.what();
        usage();
        if (parser) delete parser;
        return -1;
    }

    StoeSum stoesumfile(parser->sumfile(), parser->verbosity());
    Params params;
    if (!parser->offsetsfile().empty()) {
        params.getoffsets(parser->offsetsfile());
    }
    if (parser->stoerun() == 0) {
        // print all runs into individual subdirectories run??
        std::cout << "!---> no explicit run number given. Printing info for all runs\n";

        for (size_t r = 1; r <= stoesumfile.numRuns(); ++r) {
            RunInfo run(stoesumfile.run(r));
            std::string templ = stoesumfile.xdstemplate(r);
            run.applyOffsets(params);
            std::string dirname = io::out::outdir("run", r);
            std::ofstream outp (dirname+"/XDS.INP");
            hello(outp);
            io::out::xdsout(outp, run, params, templ);
            outp.close();
        }
    } else {
        int r = parser->stoerun();
        std::cout << "! Printing Info for Run " << r << std::endl;
        RunInfo run(stoesumfile.run(r));
        std::string templ = stoesumfile.xdstemplate(r);
        run.applyOffsets(params);
            std::string dirname = io::out::outdir("run", r);
            std::ofstream outp (dirname+"/XDS.INP");
        io::out::xdsout(outp, run, params, templ);
        outp.close();
    }// masterfile is empty, and run number explicitly given 

    delete parser;
    return EXIT_SUCCESS;
}

