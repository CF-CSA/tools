/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   Params.h
 * Author: tg
 *
 * Created on May 28, 2019, 9:31 AM
 */

#ifndef PARAMS_H
#define PARAMS_H

#include <string>
#include <list>
#include <stdexcept>


class Params {
private:
    std::string paramsfile_;
    std::string offsetsfile_;
    std::list<std::string> paramsinp_;
    float omega_;
    float twotheta_;
    float phi_;
    float chi_;
    
    //! offset values for 2nd source instrument, e.g. Stadivari
    float delta2Theta_;
    float deltaOmega_;
    float deltaChi_;
    float deltaPhi_;
    float deltaD_;
    float delta_orgx_, delta_orgy_;
    
    //! detector properties
    std::string detector_name_;
    short nx_, ny_;
    float qx_, qy_;
    
    float wavelength_;
    
    // total degree about whatever axis
    float sweep_;
    // axis (as word)
    std::string axis_;
    float detector_distance_;
    float exposure_time_;
    // from EIGER settings
    int   nimages_;
    // explicit data range
    int   data_range_[2];
    float frame_time_;
    
    std::string data_template_;
    
    // beam positions at 2theta = 0, pixels
    float beamx0_, beamy0_;
    
    // optional keywords get a flag whether they are set
    bool isset_detector_distance_;
    bool isset_exposure_time_;
    bool isset_nimages_;
    bool isset_data_range_;
    bool isset_frame_time_;
    bool isset_sweep_;
    bool isset_data_template_;
    

    std::string stripinput (std::string& inpline);
    template <typename T> bool getvalues (const std::string& keyword, short numvalues, T* values);
    
    // read parameter file for known parameters
    void readparamfile(const std::string& inpfile);
    
    // parse paramsinp_ and extract required keywords
    void getparams();

    //! get offsets, e.g. for Stadivari getoffsets
    void getoffsets();
  
public:
    // no value: offsets to be read separately
    Params();
    // one string: only parameters
    Params(const std::string& paramsfile);
    // two strings: parameters and offsets
    Params(const std::string& paramsfile, const std::string& offsetsfile);
    ~Params(){};

    //! public version of getoffsets
    void getoffsets(const std::string& offsetsfile);
    
    float omega() const { return omega_; }
    float twotheta() const { return twotheta_; }
    float phi() const { return phi_; }
    float chi() const { return chi_; }
    
    float delta2Theta() const { return delta2Theta_; }
    float deltaOmega() const { return deltaOmega_; }
    float deltaChi () const { return deltaChi_; }
    float deltaPhi() const { return deltaPhi_; }
    float deltaD() const { return deltaD_; }
    float delta_orgx() const { return delta_orgx_; }
    float delta_orgy() const { return delta_orgy_; }
    
    short nx() const { return nx_; }
    short ny() const { return ny_; }
    float qx() const { return qx_; }
    float qy() const { return qy_; }
    
    float wavelength() const { return wavelength_; }
    
    float sweep() const { return sweep_; }
    std::string axis() const { return axis_; }
    float detector_distance() const { return detector_distance_; }
    int nimages() const { return nimages_; }
    std::string data_template() const { return data_template_; }
    
    float beamx0() const { return beamx0_; }
    float beamy0() const { return beamy0_; }
    
    float oscillation_width() const;
    float rotdir() const;
    
    float angle(int thisframenum) const;
    //! helper function to ensure 0 <= angle < 360
    float anglemod (float angle) const;

};

#endif /* PARAMS_H */

