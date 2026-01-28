#include "Usage.h"

#include <iostream>
#include <cstdio>

/**
 *print a short welcome notice
 */
std::ostream& hello(std::ostream& outp) {
    outp << "!---------------------------------------------------------------!\n"
            << "! sfrmtools:    Read a STOE sum-file,                              !\n"
            << "!               extract experimental parameters and             !\n"
            << "!               print the corresponding XDS parameters.         !\n"
            << "! Copyright:    Tim Gruene, 2023                                !\n"
            << "!---------------------------------------------------------------!\n"
            ;
    return outp;
}

void usage() {
    std::cout << "!---------------------------------------------------------------!\n"
              << "! Usage: xitools -x project.sum                                 !\n"
              << "!        reads parameters from a STOE SUM file 'project.sum'    !\n"
	      << "!        and converts to XDS keywords.                          !\n"
              << "!        assumes that CBF files exist in the same directory as  !\n"
              << "!        the SUM-file                                           !\n"
              << "!         OPTIONS:                                              !\n"
	      << "!        -O offsets.txt: read offsets from file 'offsets.txt'   !\n"
	      << "!        -r RUNNUM     :  print XDS parameters for run RUNNUM\n"
              << "!        -h / -?: print this help message and exit              !\n"
	      << "\n";
   
}
