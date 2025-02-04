/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

#include "io.h"
#include "myExceptions.h"
#include "defines.h"
#include "Mat33.h"
//#include "Utils.h"
#include <iostream>
#include <iomanip>
#include <cmath>
#include <complex>
#include <iomanip>

#include <sys/stat.h>
#include <sstream>
#include <iomanip>

/**
 * Create the output directory and return its name
 * TODO: catch error codes
 * @param name
 * @param run
 * @return 
 */
std::string io::out::outdir(const std::string name, const int& run) {
    std::ostringstream outd;
    outd << name << std::setw(2) << std::setfill('0') << run;
    std::string dirname = outd.str();
    int status = mkdir(dirname.c_str(), S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH);
    
    switch (status) {
        case EACCES: 
            std::cout << "*** Error: Cannot create output directory " << dirname << std::endl;
            throw myExcepts::FileIO ("Cannot create output subdirectory");
            break;
        case EEXIST: return dirname;
    }
    
    return dirname;
    
}
void io::out::xdsout(const Params& params) {
        std::cout << " DETECTOR= EIGERR500\n"
                << " GAIN= 1.0\n"
                << " DATA_RANGE_FIXED_SCALE_FACTOR= 1 1800 1.0\n"
            << " OVERLOAD= " << (1<<20)-1
            << " MINIMUM_VALID_PIXEL_VALUE= " << 0
            << '\n';
	if (params.nx() < 0 || params.ny() < 0) {
	 std::cout << "*** Error: NX not supplied in parameter file.\n";
	 throw myExcepts::Usage("NX and NY must be specified");
	 }
          std::cout << " NX= " << params.nx()  << " NY= " << params.ny() << "\n"
            << " QX= " << params.qx() << " QY= " << params.qy() << "\n"
            << " ORGX= " << 0.5*1030 + params.delta_orgx() << " ORGY= " << 514/2 + params.delta_orgy() << '\n'
            << " TRUSTED_REGION=  0.00  1.5\n" 
            // refine defaults for IDXREF, CORRECT, but nothing for INTEGRATIO
            << "! REFINE(IDXREF)=  POSITION BEAM AXIS ORIENTATION CELL\n"
            << "  REFINE(INTEGRATE)=  !POSITION BEAM AXIS ORIENTATION CELL\n"
            << "! REFINE(CORRECT)=  POSITION BEAM AXIS ORIENTATION CELL\n"
            << "! *** warning ***: overload and minimum_valid_pixel_value hardcoded\n";

        std::cout << "! JOB= XYCORR INIT COLSPOT IDXREF DEFPIX INTEGRATE CORRECT\n";
        std::cout << " JOB= XYCORR INIT COLSPOT IDXREF DEFPIX INTEGRATE CORRECT\n";
        std::cout << " SNRC= 50\n\n";
        std::cout << " DELPHI= 25\n";
        std::cout << " INCIDENT_BEAM_DIRECTION=0.0 0.0 1.0\n";
        std::cout << " FRACTION_OF_POLARIZATION=0.50\n";
}

std::ostream& io::out::xdsout(std::ostream& outp, const RunInfo& run, 
	const Params& params, const std::string& xdstempl) {
    // assumes detector swing axis always vertical

    /*
     some comments
     */
    outp << '\n';
    outp << "! Data extracted for run number " << run.run_number_ << "\n";
    outp << "! 2theta = " << std::setprecision(1) << std::fixed << std::setw(6) 
                << run.two_theta_ << "deg, assuming vertical swing axis\n";
    outp << "! omega, chi, phi = " << run.omega_[0] << ", " 
            << run.chi_ << ", " 
            << run.phi_[0] << "\n";
    Mat33 Z = run.zeromatrix();
    outp << "! Rotation matrix R to reset unit cell axes to standard orientation\n"
            << "! at omega = 0, chi = 0. phi = 0:\n"
            << "! R * [ UNIT_CELL_A-AXIS ^T UNIT_CELL_B-AXIS^T UNIT_CELL_C-AXIS^T ]\n"
            << "!  " 
            << std::setprecision(5) << std::setw(9)
            << Z(0,0) << std::setw(9) << Z(0, 1) << std::setw(9) << Z(0,2) << "\n"
            << "!  " 
            << std::setprecision(5) << std::setw(9)
            << Z(1,0) << std::setw(9) << Z(1, 1) << std::setw(9) << Z(1,2) << "\n"
            << "!  " 
            << std::setprecision(5) << std::setw(9)
            << Z(2,0) << std::setw(9) << Z(2, 1) << std::setw(9) << Z(2,2) << " \n"
            << "! Ensure that STARTING_ANGLE=     0.000 and STARTING_FRAME= is set to \n"
            << "! the first number in DATA_RANGE= or omit 'STARTING_ANGLE entirely.\n"
            << "!  Zero-Matrix in python syntax: \n" 
            << "![["
            << std::setprecision(5) << std::setw(9)
            << Z(0,0) << ',' << std::setw(9) << Z(0, 1) << ',' << std::setw(9) << Z(0,2) << " ], ["
            << std::setprecision(5) << std::setw(9)
            << Z(1,0) << ',' << std::setw(9) << Z(1, 1) << ',' << std::setw(9) << Z(1,2) << " ], ["
            << std::setprecision(5) << std::setw(9)
            << Z(2,0) << ',' << std::setw(9) << Z(2, 1) << ','  << std::setw(9) << Z(2,2) << " ]]\n";

    if (run.omegascan_) {
        outp << "! Rotation about omega axis\n";
    }
    else if (run.phiscan_) {
        outp << "! Rotation about phi axis\n";
    }
    outp << std::endl;
    /*
     Detector information, assuming hybrid pixel detector
     */
    outp << " DETECTOR= " << run.detector_ << '\n'
            << " OVERLOAD= " << (1<<20)-1
            << " MINIMUM_VALID_PIXEL_VALUE= " << 0
            << '\n';
	    if ( params.nx() < 0) {
	     outp << " NX= " << run.nx_;
	    }
	    else {
	     outp << " NX= " << params.nx();
	    }
	    if ( params.ny() < 0) {
	     outp << " NY= " << run.ny_ << '\n';
	    }
	    else {
	     outp << " NY= " << params.ny() << '\n';
	    }
    outp << " QX=" << std::fixed << std::setprecision(4) << run.pixelsize_ << " QY=" << run.pixelsize_
            << "\n";
    
    outp << " TRUSTED_REGION=  0.00  1.5\n" 
            // refine defaults for IDXREF, CORRECT, but nothing for INTEGRATIO
            << "! REFINE(IDXREF)=  POSITION BEAM AXIS ORIENTATION CELL\n"
            << "  REFINE(INTEGRATE)=  !POSITION BEAM AXIS ORIENTATION CELL\n"
            << "! REFINE(CORRECT)=  POSITION BEAM AXIS ORIENTATION CELL\n"
            << "! *** warning ***: overload and minimum_valid_pixel_value hardcoded\n";
    // at CuKa (lambda = 1.54A) gain should be 1.0
    if (run.lambda_ > 1.5) {
        outp << " GAIN= 1.0\n";
    }
    // at MoKa (lambda = 0.713A) gain is about 0.7A, but not determined!
    else if (run.lambda_ < 0.8) {
        outp << " GAIN= 0.7\n"
                << " DATA_RANGE_FIXED_SCALE_FACTOR= 1 999 1.0\n";
    }
    // otherwise leave gain unset
    
    outp << "! JOB= XYCORR INIT COLSPOT IDXREF DEFPIX INTEGRATE CORRECT\n";
    outp << " JOB= XYCORR INIT COLSPOT IDXREF DEFPIX INTEGRATE CORRECT\n";
    outp << " SNRC= 50\n\n";
    outp << " DELPHI= 25\n";
    outp << " INCIDENT_BEAM_DIRECTION=0.0 0.0 1.0\n";
    outp << " FRACTION_OF_POLARIZATION=0.50\n";

    outp << " DIRECTION_OF_DETECTOR_X-AXIS= "
            << std::fixed
            << std::setprecision(6)
            << std::setw(9)
            << std::cos(M_PI/180.*run.two_theta_) << " 0.0 " 
                        << std::fixed
            << std::setprecision(6)
            << std::setw(9)
            << std::sin(M_PI/180.*run.two_theta_) << "\n";
    outp << " DIRECTION_OF_DETECTOR_Y-AXIS= "
            << "0.0 1.0 0.0\n";
    outp << " X-RAY_WAVELENGTH= " << run.lambda_ << "   !Angstroem\n";
    outp << " OSCILLATION_RANGE= " << std::abs(run.deltaAngle_) << " \n";

    outp << " ROTATION_AXIS= " << run.rotation_x_ << " " << run.rotation_y_
            << " " << run.rotation_z_ << "\n";

    outp << " ORGX= " << 0.5*1030 + params.delta_orgx() << " ORGY= " << 514/2 + params.delta_orgy() << '\n';
    // outp << " ORGX= " << 515 << " ORGY= " << 257 << "\n";
    outp << " DETECTOR_DISTANCE= " << std::setprecision(3) << run.delta_ << " !(mm)\n";
    
    outp << '\n';
    //! STOE tuns seem to  always start at 1
    outp << " NAME_TEMPLATE_OF_DATA_FRAMES=" << "../" << xdstempl << '\n';
    outp << " DATA_RANGE= " << 1 << " " << run.numframes()<< "\n";
    outp << '\n';
    outp << " BEAM_DIVERGENCE=   0.93746  BEAM_DIVERGENCE_E.S.D.=   0.09375" << '\n'
         << " REFLECTING_RANGE=  0.62062  REFLECTING_RANGE_E.S.D.=  0.08866" << '\n';
    
    return outp;
}

