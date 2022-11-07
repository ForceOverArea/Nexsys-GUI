from json import load, dump
from time import time
from matplotlib import pyplot as plt
from nexsys import Nexsys
from tkinter import * 
from tkinter.filedialog import askopenfilename
from tkinter.scrolledtext import ScrolledText

# import .py files for other GUIs 
from modules.file_windows import *
from modules.settings_window import *
from modules.plot_window import *

class NexsysGUI:
    """
    The Nexsys GUI window, its controls, and internal data. 
    """
    def __init__(self):
        self.window = Tk() 
        self.window.title("Nexsys")
        self.window.minsize(600, 400)
        try: self.window.iconbitmap("./images/Nexsys.ico")
        except: pass
        self.current_file = ""
        Grid.rowconfigure(self.window, 1, weight=1)
        Grid.columnconfigure(self.window, 0, weight=1)

        numcols = 7 # Hard-coded here to parametrize later code.
        color_choice = 'spring green'

        self.exprs_box = ScrolledText(
            self.window, 
            background = '#2b2b2b', 
            foreground = color_choice, 
            insertbackground = color_choice
        )
        self.exprs_box.configure(state='normal')

        # Menu Bar
        self.menubar = Menu(self.window)
        
        self.filemenu = Menu(self.menubar, tearoff = 0)
        self.filemenu.add_command(label = "New",        command = self.new_file)
        self.filemenu.add_command(label = "Open",       command = self.open_file_select)
        self.filemenu.add_command(label = "Save",       command = self.save_file)
        self.filemenu.add_command(label = "Save as...", command = self.open_saveas_window)

        self.plotmenu = Menu(self.menubar, tearoff = 0)
        self.plotmenu.add_command(label = "New 2-D Plot",           command = self.open_plot_window)
        self.plotmenu.add_command(label = "New 3-D Plot",           command = self.open_3d_plot_window)
        self.plotmenu.add_command(label = "Open Existing Plot(s)",  command = self.regen_plots)

        self.solvemenu = Menu(self.menubar, tearoff = 0)
        self.solvemenu.add_command(label = "Save & Solve",          command = self.open_solution_window)
        self.solvemenu.add_command(label = "Solve for Table",       command = self.open_table_solution_window)

        self.menubar.add_cascade(label = "File", menu = self.filemenu)
        self.menubar.add_cascade(label = "Solve", menu = self.solvemenu)
        self.menubar.add_cascade(label = "Plot", menu = self.plotmenu)
        self.menubar.add_command(label = "Settings", command = self.open_settings_window)
        self.window.config(menu = self.menubar)

        # On-screen features
        self.label =            Label( self.window, text = "No File Selected")                                      # Label showing current file

        # Orientation of On-screen features
        self.exprs_box      .grid(columnspan=numcols, column=0, row=1, padx=10, pady=10, sticky="nsew")
        self.label          .grid(columnspan = numcols, row = 2)

    def fetch_code(self) -> str:
        """Returns only the equations from the expressions box."""
        return self.exprs_box.get("0.0",END).strip()

    def open_file(self):
        """Returns the contents of a plain text file as a string."""
        with open(self.current_file, "r") as f:
            return f.read()

    # methods below this line are called by the GUI itself.
    def open_solution_window(self):
        """Show the solution to the current system."""
        self.save_file()
        sw = solution_window(self)
        sw.window.mainloop()

    def open_table_solution_window(self):
        """Show the solutions for a table of values."""
        pass

    def open_plot_window(self):
        """Create a plot of an independent and dependent variable."""
        pw = PlotWindow(self)
        pw.window.mainloop()

    def open_3d_plot_window(self):
        """Create a plot of an independent and dependent variable."""
        pw = PlotWindow(self)
        pw.window.mainloop()

    def regen_plots(self):
        """Read plot info attached to file and display it via matplotlib"""
        pass

    def open_file_select(self):
        """Open a file to edit in the Nexsys editor."""
        fp_to_open = askopenfilename(initialdir = "..", filetypes=(("Nexsys files", "*.nxs*"), ("All files", "*.*"))) # "~/Documents")
        
        if fp_to_open != "":
            self.current_file = fp_to_open

        self.label.configure(text = "Editing: " + self.current_file)
        self.exprs_box.delete("0.0",END)
        self.exprs_box.insert(END, self.open_file())

    # def edit_unit_config(self):
    #     """Edit the units.json file."""
    #     with open("settings.json", "r") as f:
    #         text_editor = load(f)["TEXT_EDITOR"]
    #     sh(f"{text_editor} ./settings/units.json")

    def new_file(self):
        """Create a new Nexsys .nxs file"""
        nfw = NewFileWindow(self)
        nfw.window.mainloop()

    def save_file(self):
        """Write the current editor window's contents to file."""
        with open(self.current_file, "w") as f:
            f.write(self.fetch_code())

    def open_saveas_window(self):
        """Write the current editor window's contents to a given filepath."""
        saw = SaveAsWindow(self)
        saw.window.mainloop()

    def open_settings_window(self):
        """Open the settings window to cleanly edit settings.json"""
        sew = SettingsWindow(self)
        sew.window.mainloop()

    def start(self):
        self.exprs_box.focus()
        self.window.mainloop()

class solution_window:
    """Window for displaying solution."""

    def __init__(self, parent: NexsysGUI):
        self.window = Toplevel()
        self.parent = parent
        self.window.title("Nexsys - Solution Window")
        try: self.window.iconbitmap("./images/Nexsys.ico")
        except: pass
        self.window.minsize(200,200)
        self.window.resizable(0, 0)

        with open("./settings/settings.json","r") as f:
            settings = load(f)
            dec_places = settings["DEC_PLACES"]
            soln_cols = settings["SOLN_COLS"]

        system = Nexsys(self.parent.fetch_code())
        try:
            start = time()
            soln = system.solve()
            duration = f"Solved in {round(time() - start, 5)} seconds."
            values = [f"{item} = {round(soln[item], dec_places)}" for item in soln]
                
            def sublists(items:list, n:int):
                # looping till length l
                for i in range(0, len(items), n): 
                    yield items[i:i + n]

            gridified = "\n\n".join(["\t\t".join(i) for i in list(sublists(values, soln_cols))])
            soln_text = f"{duration}\n=========\n\n{gridified}"
        
        except Exception as e:
            soln_text = f"Could not solve due to the following error: \n\n{str(e)}" 

        self.titlebar = Label(self.window, text = "Solution:\n=========")
        self.swtext = Label(self.window, text = soln_text)
        self.close_button = Button(self.window, text = "Close", command = self.close)

        self.titlebar       .grid(column = 0, row = 0)
        self.swtext         .grid(column = 0, row = 1, padx = 10, pady = 10)
        self.close_button   .grid(column = 0, row = 2, pady = 10)


    def close(self):
        self.window.destroy()

NexsysGUI().start()