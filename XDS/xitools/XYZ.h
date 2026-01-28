/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   XYZ.h
 * Author: tg
 *
 * Created on June 9, 2020, 8:55 AM
 */

#ifndef XYZ_H
#define XYZ_H

class Mat33;

class XYZ {
private:
    float x_, y_, z_;

public:
    XYZ() = default;
    XYZ(const float& x, const float& y, const float& z);
    ~XYZ(){}
    
    float x() const { return x_; }
    float y() const { return y_; }
    float z() const { return z_; }
    
    double norm2() const { return x_*x_ + y_*y_ + z_*z_;}
    
    XYZ unit() const;
    
    XYZ& operator+= (const XYZ& other);
    XYZ& operator*= (const float& s);
    
    friend XYZ operator* (const Mat33& R, const XYZ& xyz);
    friend XYZ operator* (const float& s, const XYZ& xyz);
    friend XYZ operator+ (const XYZ& x1, const XYZ& x2);
    friend XYZ operator- (const XYZ& x1, const XYZ& x2);
    
    friend std::ostream& operator<<(std::ostream& out, const XYZ& xyz);
    
    friend Mat33 operator*(const Mat33& m1, const Mat33& m2);
    friend XYZ operator* (const Mat33& m, const XYZ& xyz);


};

#endif /* XYZ_H */

