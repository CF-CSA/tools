/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

#include <iostream>
#include <iomanip>
#include <math.h>
#include <valarray>

#include "Utils.h"
#include "myExceptions.h"

std::string Utils::xdstemplate(const std::string root, unsigned short run, const std::string suffix) {
    if (run > 99) {
        std::cout << "*** Error: runs greater than 99 are not supported (by X-Area)\n";
        throw myExcepts::Format("Runs > 99");
    }
    std::ostringstream outp (root);
    outp << '_' << std::setw(2) << std::setfill('0') << run << "????." << suffix;
    return outp.str();
}

/**
 * Return a string containing the current date and time - 'ctime' reformatted
 * with the last carriage return removed
 */
std::string Utils::timestamp()
{
        std::time_t seconds = time(NULL);
        std::string now (std::ctime(&seconds));
        now.erase(now.end()-1);

        return now;
}

/**
 * Create the rotation matrix corresponding to a rotation about @axis by 
 * @angle. Angle expected in radians
 * @param angle
 * @param axis
 * @return 
 */
Mat33 Utils::rotaxis(const double& angle, const XYZ& axis) {
    // normalise axis - could be expected for faster code,
    // but for this project, it won't make significant difference
    XYZ n = axis.unit();
    const double nx = n.x();
    const double ny = n.y();
    const double nz = n.z();
    
    const double cos_a = std::cos(angle);
    const double sin_a = std::sin(angle);
    Mat33 R;
    R(0, 0) = nx * nx * (1.0 - cos_a) + cos_a;
    R(0, 1) = ny * nx * (1.0 - cos_a) - nz*sin_a;
    R(0, 2) = nx * nz * (1.0 - cos_a) + ny*sin_a;

    R(1, 0) = nx * ny * (1.0 - cos_a) + nz*sin_a;
    R(1, 1) = ny * ny * (1.0 - cos_a) + cos_a;
    R(1, 2) = ny * nz * (1.0 - cos_a) - nx*sin_a;

    R(2, 0) = nx * nz * (1.0 - cos_a) - ny*sin_a;
    R(2, 1) = ny * nz * (1.0 - cos_a) + nx*sin_a;
    R(2, 2) = nz * nz * (1.0 - cos_a) + cos_a;
    
    return R;
}
