from PySide6.QtWidgets import (QMainWindow, QWidget, QVBoxLayout, 
                               QHBoxLayout, QPushButton, QLabel, QTextEdit, 
                               QStatusBar)
from PySide6.QtCore import Slot, Signal
import datetime

class MainWindow(QMainWindow):
    def __init__(self, app_context):
        super().__init__()
        self.ctx = app_context
        self.setWindowTitle("SayIntentions AI (Linux Client)")
        self.resize(800, 600)

        # Central Widget
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        layout = QVBoxLayout(central_widget)

        # Top Bar
        top_layout = QHBoxLayout()
        self.status_label = QLabel("Status: Disconnected")
        self.connect_btn = QPushButton("Connect (Mock)")
        self.connect_btn.clicked.connect(self.on_connect_clicked)
        
        top_layout.addWidget(self.status_label)
        top_layout.addStretch()
        top_layout.addWidget(self.connect_btn)
        layout.addLayout(top_layout)

        # Log View
        self.log_view = QTextEdit()
        self.log_view.setReadOnly(True)
        layout.addWidget(self.log_view)

        # Sim Status
        self.sim_label = QLabel("Waiting for Simulator data...")
        layout.addWidget(self.sim_label)

    @Slot()
    def on_connect_clicked(self):
        self.log("Connecting to Mock SAPI...")
        # In a real app this would be async
        if self.ctx.sapi.connect("mock_key"):
            self.status_label.setText("Status: Connected (Mock)")
            self.connect_btn.setDisabled(True)
            self.log("Connected!")
        else:
            self.log("Connection failed.")

    def log(self, message):
        timestamp = datetime.datetime.now().strftime("%H:%M:%S")
        self.log_view.append(f"[{timestamp}] {message}")

    def update_sim_data(self, data):
        self.sim_label.setText(f"Sim Data: {data}")
