import sys
from PyQt6.QtWidgets import QApplication
from PyQt6.QtCore import Qt
from Functions.Classes.Base_UI import UIFramework


class GraphicalCalculator(UIFramework):
    def __init__(self):
        super().__init__()
        self.UI()

    def Startup():
        app = QApplication(sys.argv)
        window = GraphicalCalculator()
        window.show()
        sys.exit(app.exec())