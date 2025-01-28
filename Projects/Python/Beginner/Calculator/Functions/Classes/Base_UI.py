from PyQt6.QtWidgets import (
    QWidget, QVBoxLayout, QPushButton, QLabel, QGridLayout,
    QGraphicsBlurEffect, QMainWindow
)
from PyQt6.QtCore import Qt

class TransparentWindow(QMainWindow):
    def __init__(self, MainWindow: QMainWindow) -> None:
        super().__init__(MainWindow)
        self.__MainUI(MainWindow)
        
        
    def __MainUI(self, MainWindow):
        self.setWindowFlags(Qt.WindowType.Desktop | Qt.WindowType.FramelessWindowHint | Qt.WindowType.WindowStaysOnTopHint)
        self.setAttribute(Qt.WA_TranslucentBackground)
        self.setGraphicsEffect(QGraphicsBlurEffect())
        
        self.setCentralWidget(MainWindow)

class UIFramework(QWidget):
    def __init__(self) -> None:
        super().__init__()
        self.UI()
    
    def UI(self):
        # Declaring the overall size and shape of the UI
        self.setGeometry(100, 100, 400, 300)
        self.setWindowTitle("Graphical Calculator")
        self.show()
        
        #Transparency 
        self.setAttribute(Qt.WidgetAttribute.WA_TranslucentBackground)
        self.setStyleSheet("Background: transparent;")
                
        