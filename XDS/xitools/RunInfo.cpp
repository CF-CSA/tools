/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   RunInfo.cpp
 * Author: tg
 * 
 * Created on August 2, 2021, 12:12 PM
 */

#include "RunInfo.h"
#include "defines.h"
#include <iostream>
#include <cmath>
#include "Mat33.h"
#include "Utils.h"


RunInfo::RunInfo():
detector_ (Defaults::EIGER2::MODEL),
omegascan_ (false), phiscan_(false), nx_(1029), ny_(513), T_(100.0){
}

void RunInfo::applyOffsets(const Params& params) {
    two_theta_ += params.delta2Theta();
    omega_[0]  += params.deltaOmega();
    omega_[1]  += params.deltaOmega();
    phi_[0]    += params.deltaPhi();
    phi_[1]    += params.deltaPhi();
    chi_       += params.deltaChi();
    delta_     += params.deltaD();
    
    // once everything is corrected, calculate rotation axis
    rotationaxis();
}

float RunInfo::totalAngle() const {
    float angle;
    if (omegascan_) {
        if (phiscan_) {
            std::cout << "*** Error: both phi- and omega scan are set\n";
            throw std::logic_error("Both PHI and OMEGA scan");
        }
        angle = omega_[1] - omega_[0];
    }
    else if (phiscan_) {
        angle = phi_[1] - phi_[0];
    }
    else {
        std::cout << "*** Error: neither phi- and omega scan is set\n";
            throw std::logic_error("Neither PHI and OMEGA scan");
        
    }
    return angle;
}

void RunInfo::rotationaxis() {
    // rotation about omega is easy
    if (omegascan_) {
        rotation_x_ = 0;
        if (deltaAngle_ < 0) {
            rotation_y_ = 1;
        }
        else {
            rotation_y_ = -1;
        }
        rotation_z_ = 0;
    }
    /* phi axis:
     * (0 -1 0) = -e_y for omega = 0, chi = 0 is rotated by -chi about the axis
     * (0 0 1) = e_z rotated by omega (about (0 -1 0)) 
     */
    else if (phiscan_) {
        float rotation_direction = 1.0;
    
        if (deltaAngle_ < 0.0) {
            rotation_direction = -1.0;
        }
        rotation_x_ = rotation_direction * std::cos (M_PI/180*omega_[0])*std::sin(M_PI/180.*chi_);
        rotation_y_ = rotation_direction * std::cos (M_PI/180.*chi_);
        rotation_z_ = rotation_direction * std::sin(M_PI/180*omega_[0])*std::sin(M_PI/180.*chi_);
    }
    // should not happen, captured by  constructor
    else {
        throw std::logic_error (" ***Error: Crystal rotation about 2theta or chi not supported\n");
    }
}

/**
 * Calculate the matrix that rotates the crystal from the starting position of
 * this run (as expressed by 
 * UNIT_CELL_[ABC]-AXIS to the position where 
 * omega = 0, chi = 0, phi = 0
 * Procedure: 
 * 1) matrix of omega, as omega axis is (0 -1 0) in XDS coordinate 
 * system
 * 2) matrix for -chi. When omega=0 degree (for STOE Staedivari), chi axis is (0 0 -1)
 * , direction of beam
 * 3) matrix for -phi, as finally, with omega=0 and chi=0, phi axis is (0 -1 0)
 */
Mat33 RunInfo::zeromatrix() const{
    Mat33 Z, M;
    // rotate backwards by omega_
    const double omega = M_PI/180.*omega_[0];
    Z = Utils::rotaxis(omega, XYZ(0, -1, 0));
    // rotate backwards by chi_
    const double chi = M_PI/180.*chi_;
    M = Utils::rotaxis(chi, XYZ(0, 0, 1));
    Z = M*Z;
    const double phi = M_PI/180.*phi_[0];
    M = Utils::rotaxis(phi, XYZ(0, -1, 0));
    Z = M*Z;
    return Z;    
}
