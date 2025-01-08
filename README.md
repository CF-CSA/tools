# tools
Collection of tools and scripts

## weightedcell 
reads in one or more CORRECT.LP from XDS, computes the weighted average cell
and writes a valid XSCALE.INP to stdout. Use, e.g. as
	weightedcell ../*/CORRECT.LP | tee XSCALE.INP
Afterwards, run xscale_par
