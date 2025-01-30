/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   Params.cpp
 * Author: tg
 * 
 * Created on May 28, 2019, 9:31 AM
 */

#include "Params.h"
#include "myExceptions.h"
#include <fstream>
#include <iostream>

Params::Params() :
delta2Theta_(0.0),
deltaChi_(0.0),
deltaOmega_(0.0),
deltaPhi_(0.0),
deltaD_(0.0),
delta_orgx_(0.0),
delta_orgy_(0.0),
nx_(-1),
ny_(-1),
qx_(-1),
qy_(-1),
wavelength_(-1),
isset_detector_distance_(true),
isset_exposure_time_(true),
isset_nimages_(true),
isset_data_range_(true),
isset_frame_time_(true),
isset_sweep_(true),
isset_data_template_(true) {
}

/**
 * presets: flags of optional parameters will be set to false during reading
 * of parameter file
 * @param paramsfile
 */
Params::Params(const std::string& paramsfile) :
paramsfile_(paramsfile)
{
    Params();
    readparamfile(paramsfile_);
    getparams();
}

Params::Params(const std::string& paramsfile, const std::string& offsetsfile) :
paramsfile_(paramsfile),
offsetsfile_(offsetsfile),
        omega_(0.0),
        twotheta_(0.0),
        phi_(0.0),
        chi_(0.0),
        delta2Theta_(0.0),
        deltaChi_(0.0),
        deltaD_(0.0),
        deltaOmega_(0.0),
        deltaPhi_(0.0),
        delta_orgx_(0.0),
        delta_orgy_(0.0)
{
    Params();
    readparamfile(paramsfile_);
    getparams();

    // empty list of parameters to refill
    paramsinp_.clear();
    if (!offsetsfile_.empty()) {
        readparamfile(offsetsfile_);
        getparams();
    }
}

void Params::readparamfile(const std::string& inpfile) {
    std::ifstream inp(inpfile);
    if (!inp.is_open()) {
        throw myExcepts::FileMissing("PARAMETERS file " + inpfile + " cannot be opened");
    }
    // read XDS.INP, remove comments and check for keywords;
    const unsigned int maxlinelength(512);
    char line[maxlinelength];
    while (inp.is_open()) {
        inp.getline(line, maxlinelength);
        if (inp.fail() || inp.eof()) break;
        std::string myline(line);
        stripinput(myline);
        // check for input
        if (myline.length() > 0) {
            paramsinp_.push_back(myline);
        }
    }
    inp.close();
}

/**
 * finds first exclmation marks on input line and strips everything 
 * to its right
 * @param inpline
 * @return 
 */
std::string Params::stripinput(std::string& inpline) {

    size_t exclmark = inpline.find('!');
    if (exclmark != std::string::npos) {
        inpline = inpline.substr(0, exclmark);
    }
    return inpline;
}

void Params::getoffsets(const std::string& offsetsfile) {
    offsetsfile_ = offsetsfile;
    readparamfile(offsetsfile_);
    getoffsets();
}

void Params::getoffsets() {
    try {
        getvalues("DELTAOMEGA=", 1, &deltaOmega_);
        getvalues("DELTA2THETA=", 1, &delta2Theta_);
        getvalues("DELTAPHI=", 1, &deltaPhi_);
        getvalues("DELTACHI=", 1, &deltaChi_);
        getvalues("DELTAD=", 1, &deltaD_);
        getvalues("DELTAORGX=", 1, &delta_orgx_);
        getvalues("DELTAORGY=", 1, &delta_orgy_);
        getvalues("DETECTOR=", 1, &detector_name_);
        getvalues("NX=", 1, &nx_);
        getvalues("NY=", 1, &ny_);
        getvalues("QX=", 1, &qx_);
        getvalues("QY=", 1, &qy_);
    } catch (myExcepts::Format& e) {
        // offsets may not be given, thus ignore
    }
}

/**
 * Project specific list of keywords to be read from
 * parameter file.
 * Compulsory keywords interrupt program with error message
 */
void Params::getparams() {
    //! compulsory keaywords
    try {
        getvalues("OMEGA=", 1, &omega_);
        getvalues("2THETA=", 1, &twotheta_);
        getvalues("PHI=", 1, &phi_);
        getvalues("CHI=", 1, &chi_);
        getvalues("AXIS=", 1, &axis_);
    } catch (myExcepts::Format& e) {
        std::cout << "*** Error - missing Parmeter: " << e.what() << '\n';
        throw;

    }
    //! optional keywords

    try {
        getvalues("DETECTOR_DISTANCE=", 1, &detector_distance_);
    } catch (myExcepts::Format& e) {
        isset_detector_distance_ = false;
    }

    try {
        getvalues("SWEEP=", 1, &sweep_);
    } catch (myExcepts::Format& e) {
        isset_sweep_ = false;
    }

    try {
        getvalues("FRAME_TIME=", 1, &frame_time_);
    } catch (myExcepts::Format& e) {
        isset_frame_time_ = false;
    }

    try {
        getvalues("EXPOSURE_TIME=", 1, &exposure_time_);
    } catch (myExcepts::Format& e) {
        isset_exposure_time_ = false;
    }

    try {
        getvalues("NIMAGES=", 1, &nimages_);
    } catch (myExcepts::Format& e) {
        isset_frame_time_ = false;
    }

    try {
        getvalues("DATA_RANGE=", 2, data_range_);
    } catch (myExcepts::Format& e) {
        isset_data_range_ = false;
    }

    try {
        getvalues("NAME_TEMPLATE_OF_DATA_FRAMES=", 1, &data_template_);
    } catch (myExcepts::Format& e) {
        isset_data_template_ = false;
    }

    try {
        getvalues("X-RAY_WAVELENGTH=", 1, &wavelength_);
    } catch (myExcepts::Format& e) {
        wavelength_ = -1.0;
    }

    try {
        getvalues("BEAMX0=", 1, &beamx0_);
    } catch (myExcepts::Format& e) {
    }

    try {
        getvalues("BEAMY0=", 1, &beamy0_);
    } catch (myExcepts::Format& e) {
    }

}

/**
 * Extract values following a keyword. Several values allow per key
 * @param keyword Keyword to find
 * @param numvalues number of expected values (e.g. 6 for unit_cell_parameters)
 * @param values
 * @return 
 */
template <typename T> bool Params::getvalues(const std::string& keyword, short numvalues, T* values) {
    std::list<std::string>::const_iterator it;
    for (it = paramsinp_.begin(); it != paramsinp_.end(); ++it) {
        size_t mykey = it->find(keyword);
        // not found: next line
        if (mykey == std::string::npos) continue;

        // check if 'keyword' is only subword (e.g. ORGX= in SEGMENT_ORGX=)
        // either beginning of line, or a space before keyword
        if (mykey > 0 && it->at(mykey - 1) != ' ') continue;
        // start values just after '=' sign in keyword
        std::istringstream conv(it->substr(mykey + keyword.size() + 1, it->length()));
        for (short i = 0; i < numvalues; ++i) {
            // read into local variable and assign later to avoid overwriting 
            // in case of failure
            T localvalue;
            conv >> localvalue;
            if (conv.fail()) {
                throw myExcepts::Format(keyword.c_str());
            }
            values[i] = localvalue;
        }
        return true;
    }

    return false;
}

/**
 * oscillation width = |sweep| / exposure_time * frame_time
 * @return 
 */
float Params::oscillation_width() const {
    const float osc(frame_time_ * std::abs(sweep_) / exposure_time_);
    return osc;
}

float Params::rotdir() const {
    if (sweep_ < 0)
        return -1.0;
    else
        return 1.0;
}

float Params::angle(int thisframenum) const {
    float val(omega_);
    if (axis_ == "PHI") {
        val = phi_;
    }
    val += thisframenum * rotdir() * oscillation_width();
    return val;
}

float Params::anglemod(float angle) const {
    while (angle < 0) angle += 360.0;
    while (angle > 360.0) angle -= 360.0;
    return angle;
}
