from copy import deepcopy
from nexsys_rs import py_solve
from re import findall, IGNORECASE

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
        self.params = {}

        self.limit = 10_000
        self.tolerance = 1e-5
        self.min_delta = 1e-300

        if "limit" in kwargs: self.limit = kwargs["limit"]
        if "tolerance" in kwargs: self.tolerance = kwargs["tolerance"]
        if "min_delta" in kwargs: self.min_delta = kwargs["min_delta"]

        # Evaluate parameters; i.e. anything with a `:`
        for line in code.split("\n"):
            if ":" in line:
                terms = line.split(":")
                self.params[terms[0]] = eval(
                    terms[1],
                    deepcopy(self.params)
                )

        for char in self.params:
            code = code.replace(char, f"({self.params[char]})")

        search = "keep [a-z_]+ on [-?[0-9]+, ?-?[0-9]+]"
        for v in findall(search, code, IGNORECASE):
            
            terms = v.split() # arrange the bounding terms for interpretation
            var = terms[1]
            nums = terms[3:].split(",")

            self.bounds[var] = [ float(nums[0][1:]), float(nums[1][:-1]) ]

        self.system = "\n".join([line for line in code.split("\n") if "=" in line])

    def solve(self) -> dict:
        """
        Send code to the Nexsys Rust solver to produce a solution.
        """
        print(f"SOE: {self.system}")
        print(f"bounds: {self.bounds}")

        soln = py_solve(
            self.system, 
            self.bounds, 
            self.limit, 
            self.tolerance, 
            self.min_delta
            )

        return uar(soln, self.params) # returns solved variables AND the parameters used in the solution
        