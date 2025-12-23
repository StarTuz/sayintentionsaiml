import sys
import os
import logging
from PySide6.QtWidgets import QApplication

# Add 'src' to path so imports work
current_dir = os.path.dirname(os.path.abspath(__file__))
sys.path.append(current_dir)

from core.sapi_interface import MockSapiService
from simapi.file_watcher import SimApiWatcher
from ui.main_window import MainWindow

class AppContext:
    def __init__(self):
        self.sapi = MockSapiService()
        # Default to a local data dir in user's home
        data_dir = os.path.expanduser("~/.local/share/SayIntentionsAI")
        self.sim_watcher = SimApiWatcher(data_dir)

def main():
    logging.basicConfig(level=logging.INFO)
    
    app = QApplication(sys.argv)
    
    ctx = AppContext()
    window = MainWindow(ctx)
    
    # Wire up SimWatcher updates to UI
    # Note: In a real app we need to be careful about threading here (Qt Signals)
    # For this prototype we'll verify it works in console first or assume the watcher callback 
    # might need a thread-safe wrapper, but let's just print to stdout for now to avoid complexity in step 1.
    def on_sim_update(data):
        print(f"Sim Update: {data}")
        # window.update_sim_data(data) # Unsafe from background thread
    
    ctx.sim_watcher.on_data_update = on_sim_update
    ctx.sim_watcher.start()
    
    window.show()
    
    exit_code = app.exec()
    
    ctx.sim_watcher.stop()
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
