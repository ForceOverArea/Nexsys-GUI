from json import load

class UnitHelper:
    """
    Serializes units.json and generates new units. This may take some time.
    """
    def __init__(self):
        with open("./settings/units.json", "r") as f:
            self.data = load(f)

        self.LENGTH = self.data["LENGTH"]
        self.MASS = self.data["MASS"]
        self.MOLES = self.data["MOLES"]
        self.TIME = self.data["TIME"]
        self.FREQUENCY = self.data["FREQUENCY"]
        self.TEMPERATURE = self.data["TEMPERATURE"]
        self.TEMP_DIFFERENCE = self.data["TEMP. DIFFERENCE"]
        self.VELOCITY = self.data["VELOCITY"]
        self.AREA = self.data["AREA"]
        self.VOLUME = self.data["VOLUME"]
        self.VOLUMETRIC_FLOW = self.data["VOLUMETRIC FLOW"]
        self.FORCE = self.data["FORCE"]
        self.PRESSURE = self.data["PRESSURE"]
        self.ENERGY = self.data["ENERGY"]
        self.POWER = self.data["POWER"]
        self.VISCOSITY_DYNAMIC = self.data["VISCOSITY-DYNAMIC"]
        self.VISCOSITY_KINEMATIC = self.data["VISCOSITY-KINEMATIC"]
        self.ANGLES = self.data["ANGLES"]
        self.CHARGE = self.data["CHARGE"]
        self.ELECTRICAL_CAPACITANCE = self.data["ELECTRICAL CAPACITANCE"]
        self.DIPOLE_MOMENT = self.data["DIPOLE MOMENT"]
        self.CURRENT = self.data["CURRENT"]
        self.ELECTRICAL_RESISTANCE = self.data["ELECTRICAL RESISTANCE"]
        self.ELECTROMOTIVE_FORCE = self.data["ELECTROMOTIVE FORCE"]
        self.INDUCTANCE = self.data["INDUCTANCE"]
        self.NON_DIMENSIONAL = self.data["NON DIMENSIONAL"]
        self.MAGNETIC_FLUX_DENSITY = self.data["MAGNETIC FLUX DENSITY"]
        self.MAGNETIC_FLUX = self.data["MAGNETIC FLUX"]
        self.MAGNETIC_FIELD_STRENGTH = self.data["MAGNETIC FIELD STRENGTH"]
        self.ILLUMINANCE = self.data["ILLUMINANCE"]
        self.ILLUMINANCE_FLUX = self.data["ILLUMINANCE FLUX"]
        self.CONSTANTS = self.data["CONSTANTS"]
        self.data["TORQUE"] = self.TORQUE = {} # Create torque unit dict

        self.gen_force_units()
        self.gen_pres_torq_units()

    def gen_force_units(self):
        for t in self.TIME:
            for l in self.LENGTH:
                for m in self.MASS:
                    self.FORCE[f"{m}-{l}/{t}^2"] = (self.MASS[m] * self.LENGTH[l]) / (self.TIME[t]**2)
    
    def gen_pres_torq_units(self):
        for l in self.LENGTH:
            for f in self.FORCE:
                self.PRESSURE[f"{f}/{l}^2"] = (self.FORCE[f]) / (self.LENGTH[l]**2)
                self.TORQUE[f"{f}-{l}"] = (self.FORCE[f]) * (self.LENGTH[l])
                self.TORQUE[f"{l}-{f}"] = (self.FORCE[f]) * (self.LENGTH[l])

def convert(fro: str, to: str, uh=UnitHelper()) -> float:
    """
    Returns the conversion factor between two units
    """
    for cat in uh.data:
        if fro and to in uh.data[cat]:
            return uh.data[cat][to]/uh.data[cat][fro]