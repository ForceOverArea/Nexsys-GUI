from tkinter import *

class SaveAsWindow:
    """Window for creating files from within Nexsys."""

    def __init__(self, parent):
        self.window = Toplevel()
        self.parent = parent
        self.window.title("Nexsys - Save File As...")
        try: self.window.iconbitmap("./images/Nexsys.ico")
        except: pass
        self.window.minsize(200,50)

        self.new_filename = StringVar()

        self.filename_label =   Label(self.window, text = "File Name: ")
        self.name_box =         Entry(self.window, textvariable = self.new_filename)
        self.saveas_button =    Button(self.window, text = "Save", command = self.save_as)

        self.filename_label .grid(row = 0, column = 0)
        self.name_box       .grid(row = 0, column = 1, pady = 10, sticky = "ew")
        self.saveas_button  .grid(row = 1, columnspan = 2, padx = 10, pady = 10, sticky = "ew")

    def save_as(self):
        """Save a file to a given destination."""
        new_filename = "../" + self.name_box.get() + ".nxs"
        with open(new_filename, "w") as f:
            f.write(self.parent.fetch_code())

        self.parent.current_file = new_filename
        self.parent.label.configure(text = "Editing: " + self.parent.current_file)
        
        self.window.destroy()

class NewFileWindow:
    """Window for creating new files from within Nexsys."""

    def __init__(self, parent):
        self.window = Toplevel()
        self.parent = parent
        self.window.title("Nexsys - New File")
        try: self.window.iconbitmap("./images/Nexsys.ico")
        except: pass
        self.window.minsize(200,50)

        self.new_filename = StringVar()

        self.filename_label =   Label(self.window, text = "File Name: ")
        self.name_box =         Entry(self.window, textvariable = self.new_filename)
        self.saveas_button =    Button(self.window, text = "Create File", command = self.new_file)

        self.filename_label .grid(row = 0, column = 0)
        self.name_box       .grid(row = 0, column = 1, pady = 10, sticky = "ew")
        self.saveas_button  .grid(row = 1, columnspan = 2, padx = 10, pady = 10, sticky = "ew")

    def new_file(self):
        """Create a new file."""

        self.parent.save_file()
        self.exprs_box.delete("0.0",END)

        new_filename = "../" + self.name_box.get() + ".nxs"
        with open(new_filename, "w") as f:
            f.write("")

        self.parent.current_file = new_filename
        self.parent.label.configure(text = "Editing: " + self.parent.current_file)
        
        self.window.destroy()

# class save_as_window:
#     """Window for creating files from withing Nexsys."""

#     def __init__(self, parent):
#         self.window = Toplevel()
#         self.parent = parent
#         self.window.title("Nexsys - Save File As...")
#         try: self.window.iconbitmap("./images/Nexsys.ico")
#         except: pass
#         self.window.minsize(200,50)

#         self.new_filename = StringVar()

#         self.filename_label =   Label(self.window, text = "File Name: ")
#         self.name_box =         Entry(self.window, textvariable = self.new_filename)
#         self.saveas_button =    Button(self.window, text = "Save", command = self.save_as)

#         self.filename_label .grid(row = 0, column = 0)
#         self.name_box       .grid(row = 0, column = 1, pady = 10, sticky = "ew")
#         self.saveas_button  .grid(row = 1, columnspan = 2, padx = 10, pady = 10, sticky = "ew")


#     def save_as(self):
#         """Save a file to a given destination."""
#         new_filename = "../" + self.name_box.get() + ".nxs"
#         with open(new_filename, "w") as f:
#             f.write(self.parent.fetch_code())

#         self.parent.current_file = new_filename
#         self.parent.label.configure(text = "Editing: " + self.parent.current_file)
        
#         self.window.destroy()