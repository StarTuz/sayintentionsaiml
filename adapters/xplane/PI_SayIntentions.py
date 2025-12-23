import os
import json
import time
from threading import Thread
import xp

class PythonInterface:
    def XPluginStart(self):
        self.Name = "SayIntentions Adapter"
        self.Sig = "com.sayintentions.linux.adapter"
        self.Desc = "Adapter for SayIntentions AI Client"
        
        self.running = True
        self.data_dir = os.path.expanduser("~/.local/share/SayIntentionsAI")
        if not os.path.exists(self.data_dir):
            os.makedirs(self.data_dir)
            
        self.output_file = os.path.join(self.data_dir, "simAPI_input.json")
        
        # DataRefs
        self.dr_lat = xp.findDataRef("sim/flightmodel/position/latitude")
        self.dr_lon = xp.findDataRef("sim/flightmodel/position/longitude")
        self.dr_alt = xp.findDataRef("sim/flightmodel/position/elevation")
        self.dr_hdg = xp.findDataRef("sim/flightmodel/position/mag_psi")
        self.dr_tas = xp.findDataRef("sim/flightmodel/position/true_airspeed")
        self.dr_on_ground = xp.findDataRef("sim/flightmodel/failures/onground_all")
        
        # Start update thread
        self.thread = Thread(target=self.update_loop)
        self.thread.start()
        
        xp.log("SayIntentions Adapter Started")
        return self.Name, self.Sig, self.Desc

    def XPluginStop(self):
        self.running = False
        self.thread.join()
        xp.log("SayIntentions Adapter Stopped")

    def XPluginEnable(self):
        return 1

    def XPluginDisable(self):
        pass

    def XPluginReceiveMessage(self, inFromWho, inMessage, inParam):
        pass

    def update_loop(self):
        while self.running:
            try:
                # Read DataRefs (Must be done carefully in threads? No, XPPython3 DataRefs are usually main thread only. 
                # Actually, reading them in a thread might be unsafe or return old values.
                # Ideally we use a FlightLoop callback instead of a thread for X-Plane.)
                pass 
            except Exception:
                pass
            time.sleep(1)

    # We should use FlightLoop for thread safety in X-Plane
    def update_flight_loop(self, lastCall, now, refcon):
        if not self.running:
            return 0
            
        data = {
            "latitude": xp.getDataf(self.dr_lat),
            "longitude": xp.getDataf(self.dr_lon),
            "altitude": xp.getDataf(self.dr_alt),
            "heading": xp.getDataf(self.dr_hdg),
            "airspeed_true": xp.getDataf(self.dr_tas),
            "on_ground": xp.getDatavi(self.dr_on_ground, 0), # get first element
            "timestamp": time.time()
        }
        
        try:
            # Atomic write?
            tmp_file = self.output_file + ".tmp"
            with open(tmp_file, 'w') as f:
                json.dump(data, f)
            os.rename(tmp_file, self.output_file)
        except Exception as e:
            xp.log(f"SayIntentions Error: {e}")
            
        return 1.0 # Call again in 1 second

    def XPluginStart(self):
        self.Name = "SayIntentions Adapter"
        self.Sig = "com.sayintentions.linux.adapter"
        self.Desc = "Adapter for SayIntentions AI Client"
        
        self.running = True
        self.data_dir = os.path.expanduser("~/.local/share/SayIntentionsAI")
        if not os.path.exists(self.data_dir):
            os.makedirs(self.data_dir)
            
        self.output_file = os.path.join(self.data_dir, "simAPI_input.json")

        self.dr_lat = xp.findDataRef("sim/flightmodel/position/latitude")
        self.dr_lon = xp.findDataRef("sim/flightmodel/position/longitude")
        self.dr_alt = xp.findDataRef("sim/flightmodel/position/elevation")
        self.dr_hdg = xp.findDataRef("sim/flightmodel/position/mag_psi")
        self.dr_tas = xp.findDataRef("sim/flightmodel/position/true_airspeed")
        # vector
        self.dr_on_ground = xp.findDataRef("sim/flightmodel/failures/onground_all")
        
        xp.registerFlightLoopCallback(self.update_flight_loop, 1.0, 0)
        
        return self.Name, self.Sig, self.Desc
    
    def XPluginStop(self):
        self.running = False
        xp.unregisterFlightLoopCallback(self.update_flight_loop, 0)
