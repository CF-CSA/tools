/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   io.h
 * Author: tg
 *
 * Created on May 29, 2019, 8:48 AM
 */

#ifndef IO_H
#define IO_H

#include "RunInfo.h"
#include "Params.h"

namespace io {
    namespace out {
        // create a directory named 'name'+run
        std::string outdir (const std::string name, const int& run);
        //! print standard settings assuming EIGER2 R500
        void xdsout(const Params& params);

        //! print parameters from a RunInfo
        std::ostream& xdsout(std::ostream& outp, const RunInfo& runinfo, 
				const Params& params, const std::string& xdstempl);
    }
}

#endif /* IO_H */

