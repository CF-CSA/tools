mod xyz;
mod geom;
//read XDS_ASCII.HKL
// computation of direction cosines

#[derive(Clone)]
pub struct XDSdatum {
    h_: i16,
    k_: i16,
    l_: i16,
    I_: f32,
    sigI_: f32,
    xyzd_: [f32; 3],
    rlp_: f32,
    peak_: f32,
    corr_: f32,
    psi_: f32,
    cosines_: [f32; 6],
    deviation_: f32,
}

impl XDSdatum {
    pub fn readheader(self, filename: String, verbosity: u8) -> Option(Geom) {
       let inp = std::fs::read_to_string(filename);

    }
    pub fn readdata(self, filename: String, verbosity: u8) -> Option(Vec<XDSdatum>) {
       let inp = match std::fs::read_to_string(filename) {
       	Ok(inp) => inp;
	Err(inp) => { 
		return None;
		}
       }
       for l in inp.lines() {
       	while (l != "!END_OF_HEADER") {
		continue;
       }

    }
    fn cosines(self, U: [f32; 9], G: Geom) {
        // coordinates in reciprocal space
        let c: [f32; 3] = [0; 3];
        c[0] = h_ * U[0] + k_ * U[1] + l_ * U[2];
        c[1] = h_ * U[3] + k_ * U[4] + l_ * U[5];
        c[2] = h_ * U[6] + k_ * U[7] + l_ * U[8];
        let lc = c.uvec();

	let sinetheta = 0.5*lc*G.lambda_;
	let phi = f32::atan2( f32::sqrt ( f32::max( 0.0, 1.0 -
	sinetheta*sinetheta)), sinetheta);
    }
}
