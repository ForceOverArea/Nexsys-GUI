from json import load, dump
from os import system as sh
from time import sleep
from tkinter import *
from tkinter.filedialog import askopenfilename

class SettingsWindow:
    """Window for editing settings.json with a cleaner UI."""

    def __init__(self, parent):

        self.scale = 10 # parametrizes the scaling of the accuracy setting

        with open("./settings/settings.json", "r") as f:            
            self.settings = load(f)
        
        self.window = Toplevel()
        self.parent = parent
        self.window.title("Nexsys - Settings")
        try: self.window.iconbitmap("./images/Nexsys.ico")
        except: pass
        self.window.minsize(250,200)

        self.truncate_label =       Label(self.window, text = "Decimal Places:")
        self.truncate_slider =      Scale(self.window, from_ = 0, to = 10, tickinterval = 2, orient = "horizontal")
        self.slncols_label =        Label(self.window, text = "Sln. Window Columns:")
        self.slncols_slider =       Scale(self.window, from_ = 1, to = 5, tickinterval = 1, orient = "horizontal")
        # self.accuracy_label =       Label(self.window, text = f"Solver Accuracy (Res. = 1E-{self.scale}X):")
        # self.accuracy_slider =      Scale(self.window, from_ = 1, to = 32, tickinterval = 31, orient = "horizontal")
        self.text_editor_label =    Label(self.window, text = "Change text editor:")
        self.text_editor_box =      Button(self.window, text = "Browse", command = self.change_text_editor)
        self.unit_label =           Label(self.window, text = "Edit Units file:")
        self.unit_button =          Button(self.window, text = "Edit", command = self.edit_unit_config)
        self.apply_button =         Button(self.window, text = "Apply Changes", command = self.apply_changes)

        self.truncate_label         .grid(column = 0, row = 0, pady = 10, padx = 20)
        self.truncate_slider        .grid(column = 1, row = 0, pady = 10, padx = 20, sticky = "w")
        self.slncols_label          .grid(column = 0, row = 1, pady = 10, padx = 20)
        self.slncols_slider         .grid(column = 1, row = 1, pady = 10, padx = 20, sticky = "w")
        # self.accuracy_label         .grid(column = 0, row = 2, pady = 10, padx = 20)
        # self.accuracy_slider        .grid(column = 1, row = 2, pady = 10, padx = 20, sticky = "w")
        self.text_editor_label      .grid(column = 0, row = 3, pady = 10, padx = 20)
        self.text_editor_box        .grid(column = 1, row = 3, pady = 10, padx = 20, sticky = "w")
        self.unit_label             .grid(column = 0, row = 4, pady = 10, padx = 20)
        self.unit_button            .grid(column = 1, row = 4, pady = 10, padx = 20, sticky = "w")
        self.apply_button           .grid(columnspan = 2, row = 5, pady = 10, padx = 10, sticky = "ew")

        self.truncate_slider        .set(self.settings["DEC_PLACES"])
        self.slncols_slider         .set(self.settings["SOLN_COLS"])
        # self.accuracy_slider        .set(int(self.settings["ACCURACY"][3:])/self.scale)

    def change_text_editor(self):
        """Choose a default text editor for editing units.txt"""
        fp = askopenfilename(initialdir = "/", filetypes=(("Executables", "*.exe*"), ("All files", "*.*")))
        print(fp)
        if fp != "":
            self.settings["TEXT_EDITOR"] = fp

    def edit_unit_config(self):
        """Edit the units.json file."""
        with open("./settings/settings.json", "r") as f:
            text_editor = load(f)["TEXT_EDITOR"]
        print(text_editor)
        sh(f""""{text_editor}" ./settings/units.json""")
        
    def apply_changes(self):
        self.settings["DEC_PLACES"] = int(self.truncate_slider.get())
        self.settings["SOLN_COLS"] = int(self.slncols_slider.get())
        # self.settings["ACCURACY"] = f"1E-{self.accuracy_slider.get()*self.scale}"

        print(self.settings)

        with open("./settings/settings.json","w") as f:
            dump(self.settings, f, indent = 4)

        self.apply_button.configure(text = "Changes Applied!")
        self.apply_button.update_idletasks()
        sleep(0.5)
        self.apply_button.configure(text = "Apply Changes")
