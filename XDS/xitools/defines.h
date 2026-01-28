/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   defines.h
 * Author: tg
 *
 * Created on May 13, 2023, 9:23 PM
 */

#ifndef DEFINES_H
#define DEFINES_H

namespace Defaults {
    namespace Prog {
        const char Contact[] = "Tim Gruene, tim.gruene@univie.ac.at, (c)2023";
        const char ProgName[] = "xitools";
    }
    namespace EIGER2 {
        const std::string MODEL = "EIGER2R500";
        const float QX = 0.075; // mm pixel size
        const float QY = 0.075; // mm pixel size
        const unsigned short NX = 1030; // number of physical pixels
        const unsigned short NY = 514; // number of physical pixels
        const size_t OVERLOAD =(1<<20)-1;
    }

}

#endif /* DEFINES_H */

