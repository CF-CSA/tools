/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   XYZ.cpp
 * Author: tg
 * 
 * Created on June 9, 2020, 8:55 AM
 */

#include <iomanip>
#include <valarray>

#include "XYZ.h"
#include "Mat33.h"

XYZ::XYZ(const float& x, const float& y, const float& z): 
x_(x), y_(y), z_(z){
}

XYZ& XYZ::operator +=(const XYZ& other) {
    x_ += other.x_;
    y_ += other.y_;
    z_ += other.z_;
    return *this;
}

XYZ& XYZ::operator *=(const float& s) {
    x_ *= s;
    y_ *= s;
    z_ *= s;
    return *this;
}

XYZ operator*(const Mat33& R, const XYZ& xyz) {
    XYZ res;
    res.x_ = R.matrix_[0] * xyz.x_ + R.matrix_[1] * xyz.y_ + R.matrix_[2] * xyz.z_;
    res.y_ = R.matrix_[3] * xyz.x_ + R.matrix_[4] * xyz.y_ + R.matrix_[5] * xyz.z_;
    res.z_ = R.matrix_[6] * xyz.x_ + R.matrix_[7] * xyz.y_ + R.matrix_[8] * xyz.z_;
    return res;
}

XYZ operator*(const float& s, const XYZ& xyz) {
    XYZ res;
    res.x_ = s * xyz.x_;
    res.y_ = s * xyz.y_;
    res.z_ = s * xyz.z_;
    return res;
}

XYZ operator+(const XYZ& x1, const XYZ& x2) {
    const XYZ y(x1.x_+x2.x_, x1.y_+x2.y_, x1.z_+x2.z_);
    return y;
}

XYZ operator-(const XYZ& x1, const XYZ& x2) {
    const XYZ y(x1.x_-x2.x_, x1.y_-x2.y_, x1.z_-x2.z_);
    return y;
}

std::ostream& operator<<(std::ostream& out, const XYZ& xyz) {
    
    out << "(" << std::setw(6) << std::setprecision(3) << xyz.x_ << ", "
            << std::setw(6) << std::setprecision(3) << xyz.y_ << ", "
            << std::setw(6) << std::setprecision(3) << xyz.z_ << ")";
            
    return out;
}

XYZ XYZ::unit() const {
    double scale = this->norm2();
    scale = 1./std::sqrt(scale);
    const double x = scale*x_;
    const double y = scale*y_;
    const double z = scale*z_;
    return XYZ(x, y, z);
}