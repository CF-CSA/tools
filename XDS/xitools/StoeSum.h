/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   StoeSum.h
 * Author: tg
 *
 * Created on July 28, 2021, 5:14 PM
 */

#ifndef STOESUM_H
#define STOESUM_H

#include "RunInfo.h"

#include <string>
#include <fstream>
#include <vector>


/**
 * class for parsing .sum files from Stoe 
 */
class StoeSum {
private:
    std::string sumfile_;
    std::ifstream inp_;
    
    int runs_;
    
    unsigned char verbosity_;
 
    std::vector<RunInfo> runs_info_;
    // get the block below a 'Run #<num>' keyword

    void extractRun(const int& run, RunInfo& myrun);
    void advanceKeyword(const std::string& search);

    // find the last measurement block
    void findLastBlock();
    int getRuns();
     
public:
    StoeSum(const std::string& sumfile, unsigned char verbosity);
    ~StoeSum();
    
    size_t numRuns() const { return runs_; }
    // assume the users starts counting at 1
    RunInfo run(const int r) const { return runs_info_[r-1]; }
    // create xdstemplate from sumfile name and run number
    std::string xdstemplate (int run) const;
    // extract run number and return template from a master h5 file
    std::string xdstemplate (const std::string& masterh5file, int& run);
};

#endif /* STOESUM_H */

