/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */

/* 
 * File:   Mat33.cpp
 * Author: tg
 * 
 * Created on July 25, 2023, 9:59 PM
 */

#include "Mat33.h"

Mat33::Mat33() {
    for (int i = 0; i < 9; ++i) {
        matrix_[i] = 0.0;
    }
}

Mat33::Mat33(const double& m00, const double& m01, const double& m02,
        const double& m10, const double& m11, const double& m12,
        const double& m20, const double& m21, const double& m22) {
    matrix_[0] = m00;
    matrix_[1] = m01;
    matrix_[2] = m02;
    matrix_[3] = m10;
    matrix_[4] = m11;
    matrix_[5] = m12;
    matrix_[6] = m20;
    matrix_[7] = m21;
    matrix_[8] = m22;
}

Mat33::~Mat33() {
}

Mat33 Mat33::transpose() const {
    const Mat33 T = Mat33(matrix_[0], matrix_[3], matrix_[6], 
                          matrix_[1], matrix_[4], matrix_[7],
                          matrix_[2], matrix_[5], matrix_[8]);
    return T;
}

Mat33 operator*(const Mat33& m1, const Mat33& m2) {
    Mat33 M;
    for (int r = 0; r < 3; ++r) {
        for (int c = 0; c < 3; ++c) {
            for (int i = 0; i < 3; ++i) {
                M(r, c) += m1(r, i) * m2(i, c);
            }
        }
    }
    return M;
}