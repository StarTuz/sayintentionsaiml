from .base import IATCProvider, ATCResponse, Channel, Entity
import requests
import logging
import time

class SayIntentionsProvider(IATCProvider):
    """
    Provider for SayIntentions.AI (Legacy/Cloud).
    Wraps the original REST API logic.
    """
    
    BASE_URL = "https://apipri.stratus.ai/sapi"

    def __init__(self, api_key: str):
        self.logger = logging.getLogger("SayIntentionsProvider")
        self.api_key = api_key
        self.session = requests.Session()
        self.session.headers.update({"X-API-Key": api_key})
        self.connected = False

    def connect(self) -> bool:
        try:
            # Test connection
            resp = self.session.get(f"{self.BASE_URL}/getCommsHistory", timeout=5)
            if resp.status_code == 200:
                self.connected = True
                self.logger.info("Connected to SayIntentions Cloud")
                return True
        except Exception as e:
            self.logger.error(f"Cloud connection failed: {e}")
        self.connected = False
        return False

    def disconnect(self):
        self.connected = False

    @property
    def is_connected(self) -> bool:
        return self.connected

    def get_status(self) -> str:
        return "CONNECTED (CLOUD)" if self.connected else "DISCONNECTED"

    def say(self, text: str, voice: str = "", channel: str = "") -> ATCResponse:
        # Cloud doesn't support direct TTS trigger via 'say', it uses 'sayAs' for pilot input
        # Mapping 'say' to 'sayAs' for pilot simulation
        params = {
            "message": text,
            "channel": "COM1", # Default to COM1
            "entity": "pilot"
        }
        try:
            self.session.post(f"{self.BASE_URL}/sayAs", json=params, timeout=10)
            return ATCResponse(True)
        except Exception as e:
            return ATCResponse(False, error=str(e))

    def think(self, context: str) -> ATCResponse:
        # Cloud logic handles thinking serverside automatically after 'sayAs'
        return ATCResponse(True, data="(Cloud handles thinking)")

    def listen(self, timeout_sec: int = 5) -> ATCResponse:
        return ATCResponse(False, error="Cloud provider does not support local listening yet")
