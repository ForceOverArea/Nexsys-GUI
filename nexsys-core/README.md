# [->] Nexsys
An equation solving program written in Python and Rust 🐍🦀

Nexsys (i**N**tuitive **E**quations, e**X**pressions, and **SYS**tems) is inspired by 
Engineering Equation Solver (EES), which is a program intended for engineers 
working with thermal or fluids-heavy systems. This tool allows engineers to 
develop massive mathematical models of the systems they work with and produce 
optimized designs in less time than it might take to develop the same model 
in Python or Matlab.

The main drawback of EES is its closed-source nature and poor ability to integrate
with non-Windows machines and other software, so with an interest in simulation
and computer-aided engineering, I took it upon myself to fill the gap.

The `nexsys-core` crate makes up the collection of solving algorithms written in rust to be used by Python. The functions in this crate may not be as inviting to use as a wrapped alternative in Python.