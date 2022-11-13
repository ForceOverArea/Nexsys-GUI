# Nexsys
An equation solving program written in Python and Rust 🐍🦀

Nexsys (iNtuitive Equations, eXpressions, and SYStems) is inspired by 
Engineering Equation Solver (EES), which is a program intended for engineers 
working with thermal or fluids-heavy systems. This tool allows engineers to 
develop massive mathematical models of the systems they work with and produce 
optimized designs in less time than it might take to develop the same model 
in Python or Matlab.

The main drawback of EES is its closed-source nature and poor ability to integrate
with non-Windows machines and other software, so with an interest in simulation
and computer-aided engineering, I took it upon myself to fill the gap.

The GUI of Nexsys, unit conversion tools, and wrapper module for the solver are
all written in Python, making Nexsys relatively easy to connect with other tools 
or modify on the fly. The solver is written in Rust for ⚡blazingly fast🚀 (TM) 
equation-solving. PyO3 is used to link the two languages together and Maturin was
used to create and install the Rust solver as a Pip module.
