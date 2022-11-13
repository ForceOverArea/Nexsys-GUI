from copy import deepcopy
from json import load
from nexsys_core import py_mvnr
from re import findall, IGNORECASE
from modules.convert import convert

def uar(myDict:dict, newDict:dict):
        """Stands for 'update and return'"""
        myDict.update(newDict)
        return myDict

class Nexsys:
    """
    # Nexsys
    Python access to the Rust Nexsys equation solver.

    kwargs:
    `params`: `int` - the maximum number of iterations to make in solving a system. If this is surpassed the solution is returned with a warning.
    `tolerance`: `float` - the tolerance to TRY to hit. The solver will stop iterating if this tolerance is met. Else, warnings will be returned in addition to the estimated solution.
    `min_delta`: `float` - when the change in error dips below this value, the solver returns the estimated solution with a warning.
    """

    def __init__(self, code:str, **kwargs):
        self.bounds = {} 
        self.guesses = {}
        self.params = {}

        with open("./settings/settings.json", "r") as f:
            settings = load(f)

        self.limit = 300
        self.tolerance = 1e-5
        self.uh = None

        if "limit" in kwargs: self.limit = kwargs["limit"]
        if "tolerance" in kwargs: self.tolerance = kwargs["tolerance"]
        if "unit_helper" in kwargs: self.uh = kwargs["unit_helper"]
        
        search = "[[a-z0-9_/^\-]+->[a-z0-9_/^\-]+]"
        for uc in findall(search, code, IGNORECASE):
            fro, to = uc[1:-1].split("->")
            code = code.replace(uc, " * " + str(convert(fro, to, self.uh)))

        # Evaluate parameters; i.e. anything with a `:`
        for line in code.split("\n"):
            if ":" in line:
                terms = line.split(":")
                self.params[terms[0]] = eval(
                    terms[1],
                    deepcopy(self.params)
                )

        # Sub parameters into code prior to any further interpretation
        for char in self.params:
            code = code.replace(char, f"({self.params[char]})")

        # Establish specified guess values
        search = "guess -?[0-9]+ for [a-z_]+"
        for v in findall(search, code, IGNORECASE):
            
            print(f"found guess for {v}")
            terms = v.split() # arrange the guess terms for interpretation
            var = terms[3]
            val = terms[1]

            self.guesses[var] = float(val)

        # Establish domains for all variables
        search = "keep [a-z_]+ on [-?[0-9]+, ?-?[0-9]+]"
        for v in findall(search, code, IGNORECASE):
            
            terms = v.split() # arrange the bounding terms for interpretation
            var = terms[1]
            nums = terms[3:].split(",")

            self.bounds[var] = [ float(nums[0][1:]), float(nums[1][:-1]) ]

        # Assign default guess value to all other variables
        eqns = "\n".join([line.replace("=", "-") for line in code.split("\n") if "=" in line])
        for v in findall("[a-z_]+", eqns, IGNORECASE):
            if v not in self.guesses:
                self.guesses[v] = 1.0

        # Make list of equations to pass to nexsys-core solver function
        self.system = eqns.split("\n")

    def solve(self) -> dict:
        """
        Send code to the Nexsys Rust solver to produce a solution.
        """
        print(f"SOE: {self.system}")
        print(f"guess: {self.guesses}")
        print(f"bounds: {self.bounds}")
        print(f"tolerance: {self.tolerance}")
        print(f"Max Iter.: {self.limit}")

        soln = py_mvnr(
            self.system, 
            self.guesses,
            self.bounds, 
            self.tolerance, 
            self.limit
            )

        return uar(soln, self.params) # returns solved variables AND the parameters used in the solution
        