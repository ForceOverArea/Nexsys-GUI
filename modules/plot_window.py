from tkinter import *
from nexsys import Nexsys
from matplotlib import pyplot as plt


def f_range(start, stop, steps=8):
    step_size = (stop - start) / (steps - 1)
    return [start + step_size * i for i in range(steps)]

class PlotWindow:
    """Separate Tkinter window for plot generation."""

    def __init__(self, parent):
        self.window = Tk()
        self.parent = parent
        self.window.title("Nexsys - Create New Plot")
        self.window.resizable(0, 0)

        plot_menu = Frame(self.window)
        plot_menu.grid(padx=10, pady=10)

        dmn_start = StringVar()
        dmn_end =   StringVar()
        dmn_size =  StringVar()
        ind_var =   StringVar()
        dep_var =   StringVar()
        ptitle =    StringVar()

        # Object initialization
        self.t_label =      Label(plot_menu, text = "Plot Title")
        self.iv_label =     Label(plot_menu, text = "Independent Var.")
        self.dv_label =     Label(plot_menu, text = "Dependent Var.")
        self.dstart_label = Label(plot_menu, text = "Start of Domain")
        self.dend_label =   Label(plot_menu, text = "End of Domain")
        self.dstep_label =  Label(plot_menu, text = "No. of points")
        
        self.title =        Entry(plot_menu, width = 30, textvariable = ptitle)
        self.ind_var =      Entry(plot_menu, width = 30, textvariable = ind_var)
        self.dep_var =      Entry(plot_menu, width = 30, textvariable = dep_var)
        self.dmn_start =    Entry(plot_menu, width = 30, textvariable = dmn_start)
        self.dmn_end =      Entry(plot_menu, width = 30, textvariable = dmn_end)
        self.dmn_size =     Entry(plot_menu, width = 30, textvariable = dmn_size)

        self.plot_button = Button(plot_menu, text = "Create Plot", command = self.plot)

        # Grid spacing
        self.dstart_label   .grid(column = 0, row = 0, sticky="nsew")
        self.dend_label     .grid(column = 0, row = 1, sticky="nsew")
        self.dstep_label    .grid(column = 0, row = 2, sticky="nsew")
        self.iv_label       .grid(column = 2, row = 0, sticky="nsew")
        self.dv_label       .grid(column = 2, row = 1, sticky="nsew")
        self.t_label        .grid(column = 2, row = 2, sticky="nsew")

        self.dmn_start      .grid(column = 1, row = 0, sticky="nsew")
        self.dmn_end        .grid(column = 1, row = 1, sticky="nsew")
        self.dmn_size       .grid(column = 1, row = 2, sticky="nsew")
        self.ind_var        .grid(column = 3, row = 0, sticky="nsew")
        self.dep_var        .grid(column = 3, row = 1, sticky="nsew")
        self.title          .grid(column = 3, row = 2, sticky="nsew")

        self.plot_button.grid(columnspan = 4, row = 3, sticky="nsew")

    def plot(self):
        """Plots a given dependent variable as a function of a given dependent variable"""
        
        dmn_size = self.dmn_size.get()

        if dmn_size == "":
            dmn_size = 25
        else: 
            dmn_size = int(dmn_size)

        domain = f_range(
            float(self.dmn_start.get()), 
            float(self.dmn_end.get()), 
            dmn_size
            )
        y = []
        pb = ProgBar(dmn_size, style="basic")

        self.plot_button.configure(text=pb.show())
        for x in domain:
            eqns = self.parent.fetch_code()
            system = Nexsys(eqns.replace(self.ind_var.get(), str(x)))
            y.append(system.solve()[self.dep_var.get()])
            
            pb.increment()
            self.plot_button.configure(text = pb.show())
            self.plot_button.update_idletasks()

        self.plot_button.configure(text="Create Plot")
        plt.plot(domain, y)
        plt.title(self.title.get())
        plt.show()


class ProgBar:
    """Generates a progress bar similar to the zypper package manager's."""
    def __init__(self, max_prog:int, style="basic"):
        self.style = style
        self.progress = 0.0
        self.max_prog = max_prog
        self.wheel = 0


    def increment(self, amount=1):
        self.progress += amount
        if self.wheel == 15:
            self.wheel = 0
        else: 
            self.wheel += 1


    def show(self, length=40, show_wheel=True):
        proportion = int(length*self.progress/self.max_prog)

        c = int(100*self.progress/self.max_prog)
        p = proportion + 1-len(str(c))
        f = length - proportion

        if show_wheel:
            wheel = ['/','/','/','/','-','-','-','-','\\','\\','\\','\\','|','|','|','|'][self.wheel]

        present = f'( {wheel} )'
        past = '|' * p
        future = '.' * f

        state = f"[{past}{present}{future}] {int(self.progress)}/{self.max_prog} [{c}%]"

        return state